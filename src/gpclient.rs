use crate::types::*;
use anyhow::{anyhow, Context};
use async_stream::try_stream;
use bytes::Bytes;
use futures_core::stream::Stream;
use log::{debug, info};
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RefreshToken, RevocationUrl, Scope, StandardTokenResponse, TokenResponse, TokenUrl,
};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::time;
use tokio::sync::Mutex;
use url::Url;

#[derive(serde::Deserialize)]
pub struct InstalledJs {
    installed: SecretJs,
}
#[derive(serde::Deserialize)]
struct SecretJs {
    client_id: String,
    auth_uri: String,
    token_uri: String,
    client_secret: String,
}

// Authentication token that takes care of refreshing itself.
pub struct AuthToken {
    pub token: String,
    expires_at: std::time::Instant,
    refresh_token: String,
    client_secret: String,
}
impl AuthToken {
    pub fn new(refresh_token: &str, client_secret: &str) -> AuthToken {
        AuthToken {
            token: "".to_string(),
            expires_at: time::Instant::now(),
            refresh_token: refresh_token.to_string(),
            client_secret: client_secret.to_string(),
        }
    }

    pub async fn check_token(&mut self) -> anyhow::Result<()> {
        if self.expires_at - time::Instant::now() > time::Duration::from_secs(600) {
            return Ok(());
        }

        info!("refreshing auth token");
        let js = fs::read_to_string(&self.client_secret)?;
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
        debug!("refresh response: {:?}", refresh_r);
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
    pub async fn new_from_file(client_secret: &str, auth_file: &str) -> anyhow::Result<Self> {
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
            client_secret,
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
        album_id: &GPhotoAlbumId,
    ) -> impl Stream<Item = anyhow::Result<gphotos_api::models::MediaItem>> + '_ {
        let album_id = album_id.0.clone();
        try_stream! {
            let mut token: Option<String> = None;
            loop {
                let config = self.get_config().await?;
                let search_req = gphotos_api::models::SearchMediaItemsRequest{
                    album_id: album_id.to_string(),
                    page_size: Some(100),
                    page_token: token,
                };
                debug!("requesting new page");
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
        media_item: &gphotos_api::models::MediaItem,
    ) -> anyhow::Result<Bytes> {
        let config = self.get_config().await?;
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
        Ok(bytes)
    }

    pub async fn get_album(
        &self,
        album_id: &GPhotoAlbumId,
    ) -> anyhow::Result<gphotos_api::models::Album> {
        let config = self.get_config().await?;
        gphotos_api::apis::default_api::get_album(&config, &album_id.0)
            .await
            .with_context(|| format!("failed to get album id {}", album_id))
    }
}

pub async fn get_auth(client_secret: &str, auth_file: &str) -> anyhow::Result<()> {
    let js = fs::read_to_string(client_secret)?;
    let secret_js = serde_json::from_str::<InstalledJs>(&js)?.installed;

    let google_client_id = ClientId::new(secret_js.client_id);
    let google_client_secret = ClientSecret::new(secret_js.client_secret);
    let auth_url = AuthUrl::new(secret_js.auth_uri).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(secret_js.token_uri).expect("Invalid token endpoint URL");

    // Set up the config for the Google OAuth2 process.
    let client = BasicClient::new(google_client_id)
        .set_client_secret(google_client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
        )
        // Google supports OAuth 2.0 Token Revocation (RFC-7009)
        .set_revocation_url(
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
                .expect("Invalid revocation endpoint URL"),
        );

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/photoslibrary.readonly".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!("Open this URL in your browser:\n{authorize_url}\n");

    let (code, state) = {
        // A very naive implementation of the redirect server.
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        // The server will terminate itself after collecting the first code.
        let Some(mut stream) = listener.incoming().flatten().next() else {
            panic!("listener terminated without accepting a connection");
        };

        let mut reader = BufReader::new(&stream);

        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        let redirect_url = request_line.split_whitespace().nth(1).unwrap();
        let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

        let code = url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
            .unwrap();

        let state = url
            .query_pairs()
            .find(|(key, _)| key == "state")
            .map(|(_, state)| CsrfToken::new(state.into_owned()))
            .unwrap();

        let message = "Go back to your terminal :)";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes()).unwrap();

        (code, state)
    };

    println!("Google returned the following code:\n{}\n", code.secret());
    println!(
        "Google returned the following state:\n{} (expected `{}`)\n",
        state.secret(),
        csrf_state.secret()
    );
    if state.secret() != csrf_state.secret() {
        return Err(anyhow::anyhow!("secrets do not match"));
    }

    // Exchange the code with a token.
    let token_response = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(&http_client)
        .await?;

    let token_ser = serde_json::to_string(&token_response)?;
    fs::write(auth_file, &token_ser)?;
    println!("auth token saved to {}", auth_file);
    Ok(())
}
