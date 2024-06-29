use anyhow::{anyhow, Context};
use async_stream::try_stream;
use futures::StreamExt;
use futures_core::stream::Stream;
use oauth2::basic::BasicClient;
use oauth2::{
    ClientId, ClientSecret, RefreshToken, StandardTokenResponse, TokenResponse, TokenUrl,
};
use std::fs;
use std::sync::{Arc, Mutex};
use std::time;

#[derive(serde::Deserialize)]
struct InstalledJs {
    installed: SecretJs,
}
#[derive(serde::Deserialize)]
struct SecretJs {
    client_id: String,
    token_uri: String,
    client_secret: String,
}

pub struct AuthToken {
    pub token: String,
    expires_at: std::time::Instant,
    refresh_token: String,
}
impl AuthToken {
    pub fn new(refresh_token: &str) -> AuthToken {
        AuthToken {
            token: "".to_string(),
            expires_at: time::Instant::now(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub async fn check_token(&mut self) -> anyhow::Result<()> {
        if self.expires_at - time::Instant::now() > time::Duration::from_secs(600) {
            return Ok(());
        }

        println!("refreshing auth token");
        let js = fs::read_to_string("client-secret.json")?;
        let secret_js = serde_json::from_str::<InstalledJs>(&js)?.installed;

        let google_client_id = ClientId::new(secret_js.client_id);
        let google_client_secret = ClientSecret::new(secret_js.client_secret);
        let token_url = TokenUrl::new(secret_js.token_uri).expect("Invalid token endpoint URL");

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .with_context(|| format!("Client should build"))?;

        let client = BasicClient::new(google_client_id)
            .set_token_uri(token_url)
            .set_client_secret(google_client_secret);
        let refresh_r = client
            .exchange_refresh_token(&RefreshToken::new(self.refresh_token.clone()))
            .request_async(&http_client)
            .await
            .with_context(|| format!("refresh token failed"))?;
        println!("refresh response: {:?}", refresh_r);
        self.token = refresh_r.access_token().secret().clone();
        self.expires_at = time::Instant::now()
            + refresh_r
                .expires_in()
                .unwrap_or(time::Duration::from_secs(3600));

        Ok(())
    }
}

#[derive(Clone)]
pub struct GPClient {
    token: Arc<Mutex<AuthToken>>,
    api_config: gphotos_api::apis::configuration::Configuration,
}
impl GPClient {
    pub async fn new_from_file(auth_file: &str) -> anyhow::Result<Self> {
        // We only need the refresh token.
        let saved_token: StandardTokenResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        > = serde_json::from_str(&(fs::read_to_string(auth_file)?))?;
        let token = AuthToken::new(
            saved_token
                .refresh_token()
                .ok_or(anyhow::anyhow!("can't find refresh token"))?
                .secret(),
        );

        let api_config = gphotos_api::apis::configuration::Configuration {
            oauth_access_token: Some(token.token.clone()),
            ..Default::default()
        };

        Ok(GPClient {
            token: Arc::new(Mutex::new(token)),
            api_config,
        })
    }
    async fn get_config(&self) -> anyhow::Result<gphotos_api::apis::configuration::Configuration> {
        let mut t = self.token.lock().unwrap();
        t.check_token().await?;
        Ok(gphotos_api::apis::configuration::Configuration {
            oauth_access_token: Some(t.token.clone()),
            ..self.api_config.clone()
        })
    }
    pub fn album_items_stream(
        &self,
        album_id: &str,
    ) -> impl Stream<Item = anyhow::Result<gphotos_api::models::MediaItem>> + '_ {
        let album_id = album_id.to_string().clone();
        try_stream! {
            let mut token: Option<String> = None;
            loop {
                let config = self.get_config().await?;
                let search_req = gphotos_api::models::SearchMediaItemsRequest{
                    album_id: album_id.to_string(),
                    page_size: Some(100),
                    page_token: token,
                };
                println!("requesting new page");
                let r = gphotos_api::apis::default_api::search_media_items(&config, Some(search_req)).await?;
                match r.media_items {
                    Some(media_items) => {
                        for media_item in media_items {
                            yield media_item;
                        }
                    }
                    None => break
                }
                token = r.next_page_token;
                if token.is_none() {
                    break;
                }
            }
        }
    }

    pub fn albums_stream(
        &self,
    ) -> impl Stream<Item = anyhow::Result<gphotos_api::models::Album>> + '_ {
        try_stream! {
            let mut token: Option<String> = None;
            loop {
                let config = self.get_config().await?;
                let r = gphotos_api::apis::default_api::list_albums(&config, Some(50), token.as_deref()).await?;
                match r.albums {
                    Some(albums) => {
                        for album in albums {
                            yield album;
                        }
                    }
                    None => break
                }
                token = r.next_page_token;
                if token.is_none() {
                    break;
                }
            }
        }
    }

    pub fn shared_albums_stream(
        &self,
    ) -> impl Stream<Item = anyhow::Result<gphotos_api::models::Album>> + '_ {
        try_stream! {
            let mut token: Option<String> = None;
            loop {
                let config = self.get_config().await?;
                let r = gphotos_api::apis::default_api::list_shared_albums(&config, Some(50), token.as_deref()).await?;
                match r.shared_albums {
                    Some(albums) => {
                        for album in albums {
                            yield album;
                        }
                    }
                    None => break
                }
                token = r.next_page_token;
                if token.is_none() {
                    break;
                }
            }
        }
    }

    pub fn media_items_stream(
        &self,
    ) -> impl Stream<Item = anyhow::Result<gphotos_api::models::MediaItem>> + '_ {
        try_stream! {
            let mut token: Option<String> = None;
            loop {
                let config = self.get_config().await?;
                let r = gphotos_api::apis::default_api::list_media_items(&config, Some(100), token.as_deref()).await?;
                match r.media_items {
                    Some(media_items) => {
                        for media_item in media_items {
                            yield media_item;
                        }
                    }
                    None => break
                }
                token = r.next_page_token;
                if token.is_none() {
                    break;
                }
            }
        }
    }
}
