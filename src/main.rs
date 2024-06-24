use anyhow::anyhow;
use anyhow::Context;
use async_stream::try_stream;
use futures::pin_mut;
use futures_core::stream::Stream;
use gphotos_api::apis::default_api::ListAlbumsError;
use openapi::apis::assets_api;
use openapi::apis::configuration;
use std::env;
use std::fs;
use std::time;
use tokio_stream::StreamExt;

use oauth2::basic::BasicClient;
use oauth2::{RefreshToken, StandardTokenResponse, TokenResponse};

use oauth2::{ClientId, ClientSecret, TokenUrl};

#[derive(serde::Deserialize)]
struct InstalledJs {
    installed: SecretJs,
}
#[derive(serde::Deserialize)]
struct SecretJs {
    client_id: String,
    auth_uri: String,
    token_uri: String,
    client_secret: String,
}

struct AuthToken {
    token: String,
    expires_at: std::time::Instant,
    refresh_token: String,
}
impl AuthToken {
    fn new(refresh_token: &str) -> AuthToken {
        AuthToken {
            token: "".to_string(),
            expires_at: time::Instant::now(),
            refresh_token: refresh_token.to_string(),
        }
    }

    async fn check_token(&mut self) -> anyhow::Result<()> {
        if self.expires_at - time::Instant::now() > time::Duration::from_secs(60) {
            return Ok(());
        }

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

struct GPClient {
    token: AuthToken,
    api_config: gphotos_api::apis::configuration::Configuration,
}
impl GPClient {
    fn get_config(&self) -> gphotos_api::apis::configuration::Configuration {
        gphotos_api::apis::configuration::Configuration {
            oauth_access_token: Some(self.token.token.clone()),
            ..self.api_config.clone()
        }
    }
    fn albums_stream_fn(
        &self, // api_config: &gphotos_api::apis::configuration::Configuration,
    ) -> impl Stream<
        Item = Result<
            gphotos_api::models::Album,
            gphotos_api::apis::Error<gphotos_api::apis::default_api::ListAlbumsError>,
        >,
    > + '_ {
        try_stream! {
            let mut token: Option<String> = None;
            loop {
                let r = gphotos_api::apis::default_api::list_albums(&self.get_config(), Some(50), token.as_deref()).await?;
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let auth_file = "auth_token.json";
    // We only need the refresh token.
    let saved_token: StandardTokenResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    > = serde_json::from_str(&(fs::read_to_string(auth_file)?))?;
    let mut token = AuthToken::new(
        saved_token
            .refresh_token()
            .ok_or(anyhow::anyhow!("can't find refresh token"))?
            .secret(),
    );
    token.check_token().await?;

    let gp_api_config = gphotos_api::apis::configuration::Configuration {
        oauth_access_token: Some(token.token.clone()),
        ..Default::default()
    };

    let gpclient = GPClient {
        token,
        api_config: gp_api_config,
    };
    let s = gpclient.albums_stream_fn();
    pin_mut!(s);
    while let Some(album) = s.next().await {
        let album = album?;
        println!("album {}", album.title.unwrap_or("no name".to_string()));
    }

    // let mut api_config = configuration::Configuration::new();
    // api_config.api_key =
    //     env::vars()
    //         .find(|(k, _)| k == "API_KEY")
    //         .map(|(_, v)| configuration::ApiKey {
    //             prefix: None,
    //             key: v,
    //         });
    //
    // api_config.base_path = "http://h4:2283/api".to_string();
    //
    // let r = assets_api::get_random(&api_config, Some(1.0))
    //     .await
    //     .unwrap();
    // println!("{:?}", r[0]);
    //
    Ok(())
}
