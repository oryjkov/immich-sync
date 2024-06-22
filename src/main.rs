use dotenvy::dotenv;
use std::env;
use std::fs;

use openapi::apis::assets_api;
use openapi::apis::configuration;

use oauth2::basic::BasicClient;
use oauth2::{StandardTokenResponse, TokenResponse};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let auth_file = "auth_token.json";
    let token: StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType> =
        serde_json::from_str(&(fs::read_to_string(auth_file)?))?;

    let mut gp_api_config = gphotos_api::apis::configuration::Configuration {
        oauth_access_token: Some(token.access_token().secret().clone()),
        ..Default::default()
    };
    let mut r = gphotos_api::apis::default_api::list_albums(&gp_api_config, Some(50), None).await;
    if r.is_err() {
        let js = fs::read_to_string("client-secret.json")?;
        let secret_js = serde_json::from_str::<InstalledJs>(&js)?.installed;

        let google_client_id = ClientId::new(secret_js.client_id);
        let google_client_secret = ClientSecret::new(secret_js.client_secret);
        let token_url = TokenUrl::new(secret_js.token_uri).expect("Invalid token endpoint URL");

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let client = BasicClient::new(google_client_id)
            .set_token_uri(token_url)
            .set_client_secret(google_client_secret);
        let refresh_r = client
            .exchange_refresh_token(token.refresh_token().unwrap())
            .request_async(&http_client)
            .await;
        println!("refresh response: {:?}", refresh_r);
        gp_api_config.oauth_access_token = Some(refresh_r.unwrap().access_token().secret().clone());
        r = gphotos_api::apis::default_api::list_albums(&gp_api_config, Some(50), None).await;
    }
    let r = r?;

    println!("got {} albums", r.albums.map_or(0, |a| a.len()));

    dotenv().expect(".env file not found");

    let mut api_config = configuration::Configuration::new();
    api_config.api_key =
        env::vars()
            .find(|(k, _)| k == "API_KEY")
            .map(|(_, v)| configuration::ApiKey {
                prefix: None,
                key: v,
            });

    api_config.base_path = "http://h4:2283/api".to_string();

    let r = assets_api::get_random(&api_config, Some(1.0))
        .await
        .unwrap();
    println!("{:?}", r[0]);

    Ok(())
}
