use anyhow::{anyhow, Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
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
    //

    Ok(())
}
