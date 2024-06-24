use anyhow::anyhow;
use anyhow::Context;
use futures::task::Poll;
use futures_util::pin_mut;
use gphotos_api::apis::default_api::ListAlbumsError;
use gphotos_api::models::Album; //apis::mode default_api::list_albums(&self.api_config, Some(5), None).await?;
use gphotos_api::models::ListAlbumsResponse;
use openapi::apis::assets_api;
use openapi::apis::configuration;
use std::env;
use std::fs;
use std::future::Future;
use std::pin::Pin;
use std::time;
use stream::*;
use tokio_stream::{Stream, StreamExt};

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

struct GPWrapper {
    api_config: gphotos_api::apis::configuration::Configuration,
}

// impl GPWrapper {
//     async fn get_albums(&self) -> impl Stream<Item = anyhow::Result<Album>> {
//         // TODO: not clone here
//         let x = ResultsStream::new(self.api_config.clone());
//
//         x
//     }
// }

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

    let mut albums_stream = ResultsStream::new(|token: Option<String>| {
        let c = gp_api_config.clone();
        async move {
            // TODO: how can I avoid this dance with converting String to str?
            let r = if let Some(t) = token {
                gphotos_api::apis::default_api::list_albums(&c, Some(5), Some(t.as_str())).await?
            } else {
                gphotos_api::apis::default_api::list_albums(&c, Some(5), None).await?
            };

            // TODO: seems like next token make it go in circles through the same albums!
            match r.albums {
                None => Ok::<
                    Option<Page<String, gphotos_api::models::Album>>,
                    gphotos_api::apis::Error<ListAlbumsError>,
                >(None),
                Some(albums) => {
                    if albums.len() == 0 {
                        Ok(None)
                    } else {
                        Ok(Some(Page::new(albums.into(), r.next_page_token)))
                    }
                }
            }
        }
    });
    while let Some(album) = albums_stream.next().await {
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
