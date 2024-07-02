use anyhow::{anyhow, Context};
use async_stream::try_stream;
use bytes::Bytes;
use futures_core::stream::Stream;
use oauth2::basic::BasicClient;
use oauth2::{
    ClientId, ClientSecret, RefreshToken, StandardTokenResponse, TokenResponse, TokenUrl,
};
use std::fs;
use std::sync::Arc;
use std::time;
use tokio::sync::Mutex;

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

// Authentication token that takes care of refreshing itself.
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
                .ok_or(anyhow!("can't find refresh token"))?
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
        let mut t = self.token.lock().await;
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
    pub async fn fetch_media_item(
        &self,
        gphoto_id: &str,
    ) -> anyhow::Result<(gphotos_api::models::MediaItem, Bytes)> {
        let config = self.get_config().await?;
        let media_item = gphotos_api::apis::default_api::get_media_item(&config, gphoto_id)
            .await
            .with_context(|| format!("failed to get media item id {}", gphoto_id))?;
        let metadata = media_item
            .media_metadata
            .as_ref()
            .ok_or(anyhow!(format!("missing media metadata from response")))?;
        let suffix = if metadata.photo.is_some() {
            "=d"
        } else if metadata.video.is_some() {
            "=dv"
        } else {
            Err(anyhow!("neither photo nor video"))?
        };
        let base_url = media_item
            .base_url
            .as_ref()
            .ok_or(anyhow!(format!("missing base url")))?;
        let fetch_url = format!("{}{}", base_url, suffix);

        let bytes = config
            .client
            .get(fetch_url)
            .timeout(time::Duration::from_secs(300))
            .send()
            .await?
            .bytes()
            .await?;
        Ok((media_item, bytes))
    }

    pub async fn get_album(&self, album_id: &str) -> anyhow::Result<gphotos_api::models::Album> {
        let config = self.get_config().await?;
        gphotos_api::apis::default_api::get_album(&config, album_id)
            .await
            .with_context(|| format!("failed to get album id {}", album_id))
    }
}
