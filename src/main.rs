use anyhow::{anyhow, Context};
use async_stream::try_stream;
use clap::Parser;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use futures::StreamExt;
use futures_core::stream::Stream;
use oauth2::basic::BasicClient;
use oauth2::{
    ClientId, ClientSecret, RefreshToken, StandardTokenResponse, TokenResponse, TokenUrl,
};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Row, Sqlite};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    destination: String,

    #[arg(long, default_value_t = 10)]
    concurrency: usize,
}

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
struct GPClient {
    token: Arc<Mutex<AuthToken>>,
    api_config: gphotos_api::apis::configuration::Configuration,
}
impl GPClient {
    async fn get_config(&self) -> anyhow::Result<gphotos_api::apis::configuration::Configuration> {
        let mut t = self.token.lock().unwrap();
        t.check_token().await?;
        Ok(gphotos_api::apis::configuration::Configuration {
            oauth_access_token: Some(t.token.clone()),
            ..self.api_config.clone()
        })
    }
    fn album_items_stream(
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

    fn albums_stream(&self) -> impl Stream<Item = anyhow::Result<gphotos_api::models::Album>> + '_ {
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

    fn shared_albums_stream(
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

    fn media_items_stream(
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

async fn add_album(
    album: gphotos_api::models::Album,
    is_shared: bool,
    pool: &Pool<Sqlite>,
) -> anyhow::Result<()> {
    let id = album.id.as_ref().ok_or(anyhow!(format!("no album id!")))?;

    let share_info = album
        .shared_album_options
        .as_ref()
        .map(|x| {
            let z = serde_json::to_string(x.into());
            z.ok()
        })
        .flatten();

    let media_items_count = album
        .media_items_count
        .map(|s| s.parse::<i64>())
        .transpose()?;
    let existing_row = sqlx::query(r#"SELECT id FROM albums WHERE id = $1"#)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    if existing_row.is_some() {
        // TODO: maybe just return Ok(()) ?
        return Err(anyhow!(format!("already there")));
    }

    sqlx::query(
        r#"
            INSERT INTO albums (id, title, share_info, media_items_count, cover_photo_media_item_id, is_shared) VALUES
                    ($1, $2, $3, $4, $5, $6)
            "#,
    )
    .bind(id)
    .bind(album.title)
    .bind(&share_info)
    .bind(media_items_count)
    .bind(album.cover_photo_media_item_id)
    .bind(is_shared)
    .execute(pool)
    .await?;
    Ok(())
}

// Takes a media_item description, downloads it, saves it to a local file and updates sql with
// the new record.
async fn fetch(
    media_item: &gphotos_api::models::MediaItem,
    pool: &Pool<Sqlite>,
    local_dir: &Path,
) -> anyhow::Result<()> {
    let base_url = media_item
        .base_url
        .as_ref()
        .ok_or(anyhow!(format!("missing base url")))?;
    let metadata = media_item
        .media_metadata
        .as_ref()
        .ok_or(anyhow!(format!("no metadata")))?;
    let id = media_item.id.as_ref().ok_or(anyhow!(format!("no id")))?;
    let filename = media_item.filename.clone().unwrap_or("".to_string());
    let metadata_str = serde_json::to_string(&metadata)?;
    let contributor_str = media_item
        .contributor_info
        .as_ref()
        .map(|x| {
            let z = serde_json::to_string(x.into());
            z.ok()
        })
        .flatten()
        .unwrap_or("".to_string());

    let existing_row = sqlx::query(r#"SELECT id FROM media_items WHERE id = $1"#)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    if existing_row.is_some() {
        return Ok(());
    }

    let suffix = if metadata.photo.is_some() {
        "=d"
    } else if metadata.video.is_some() {
        "=dv"
    } else {
        Err(anyhow!("neither photo nor video"))?
    };
    let fetch_url = format!("{}{}", base_url, suffix);
    //println!("going to fetch {:?}", media_item);
    let client = reqwest::Client::new();

    let bytes = client
        .get(fetch_url)
        .timeout(time::Duration::from_secs(300))
        .send()
        .await?
        .bytes()
        .await?;
    println!("fetched {} bytes for id {:?}", bytes.len(), media_item.id);

    let mut hasher = Sha1::new();
    hasher.input_str(id);
    let hex = hasher.result_str();

    let local_path = Path::new("media_items").join(&hex[0..2]).join(&hex[2..4]);
    fs::create_dir_all(local_dir.join(&local_path))?;
    let local_path = local_path.join(format!("{}.media_item", id));
    let full_path = local_dir.join(&local_path);
    let local_path = local_path.to_str().unwrap();

    tokio::fs::write(&full_path, bytes)
        .await
        .with_context(|| format!("failed writing local file {:?}", &full_path))?;

    sqlx::query(
        r#"
            INSERT INTO media_items (id, filename, local_file, description, mime_type, contributor, metadata) VALUES
                    ($1, $2, $3, $4, $5, $6, $7)
            "#,
    )
    .bind(id)
    .bind(&filename)
    .bind(&local_path)
    .bind(&media_item.description)
    .bind(&media_item.mime_type)
    .bind(&contributor_str)
    .bind(&metadata_str)
    .execute(pool)
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(
            Path::new(&args.destination)
                .join("sqlite.db")
                .to_str()
                .unwrap(),
        )
        .await?;

    let auth_file = "auth_token.json";
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

    let gp_api_config = gphotos_api::apis::configuration::Configuration {
        oauth_access_token: Some(token.token.clone()),
        ..Default::default()
    };

    let gpclient = GPClient {
        token: Arc::new(Mutex::new(token)),
        api_config: gp_api_config,
    };

    let shared_albums_stream = gpclient.shared_albums_stream().map(|album_or| {
        let p = pool.clone();
        async move { add_album(album_or?, true, &p).await }
    });
    let shared_albums_results = shared_albums_stream
        .buffer_unordered(args.concurrency)
        .collect::<Vec<_>>()
        .await;
    println!(
        "shared albums: {:?}",
        shared_albums_results
            .into_iter()
            .filter(|x| x.is_err())
            .collect::<Vec<_>>()
    );

    let albums_stream = gpclient
        .albums_stream()
        // .take(5)
        .map(|album_or| {
            let p = pool.clone();
            async move { add_album(album_or?, false, &p).await }
        });
    let albums_results = albums_stream
        //.chain(shared_albums_stream)
        .buffer_unordered(args.concurrency)
        .collect::<Vec<_>>()
        .await;
    println!(
        "albums {:?}",
        albums_results
            .into_iter()
            .filter(|x| x.is_err())
            .collect::<Vec<_>>()
    );

    let path = Path::new(&args.destination);

    // Get album mappings
    let all_albums = sqlx::query(r#"SELECT id, title, media_items_count FROM albums"#)
        .fetch_all(&pool)
        .await?;
    let album_items = futures::stream::iter(all_albums)
        // .take(1)
        .map(|row| {
            let gpclient = gpclient.clone();
            let pool = pool.clone();
            async move {
                let album_id = row.try_get("id").unwrap();
                println!(
                    "fetching items for album '{}', expecting {} items",
                    row.try_get("title").unwrap_or("----".to_string()),
                    row.try_get("media_items_count").unwrap_or(-1)
                );
                let album_items = gpclient
                    .album_items_stream(album_id)
                    .map(|media_item_or| {
                        let pool = pool.clone();
                        async move {
                            let media_item = media_item_or?;
                            fetch(&media_item, &pool, path).await?;
                            let id = media_item.id.ok_or(anyhow!("no media id!"))?;
                            Ok::<_, anyhow::Error>(id)
                        }
                    })
                    .buffer_unordered(args.concurrency)
                    .collect::<Vec<_>>()
                    .await
                    .into_iter()
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();
                println!(
                    "album '{}' done",
                    row.try_get("title").unwrap_or("----".to_string())
                );
                (album_id.to_string(), album_items)
            }
        })
        .buffer_unordered(1) // it did not work well with more than 1 concurrent request.
        .then(|(album_id, album_items)| {
            let pool = pool.clone();
            async move {
                let mut tx = pool.begin().await?;

                sqlx::query(
                    r#"
            DELETE FROM album_items WHERE album_id = $1
            "#,
                )
                .bind(&album_id)
                .execute(&mut *tx)
                .await?;

                for id in &album_items {
                    sqlx::query(
                        r#"
            INSERT INTO album_items (album_id, media_item_id) VALUES
                    ($1, $2)
            "#,
                    )
                    .bind(&album_id)
                    .bind(id)
                    .execute(&mut *tx)
                    .await?;
                }
                tx.commit().await?;
                Ok(())
            }
        })
        .collect::<Vec<_>>()
        .await;
    println!(
        "{:?}",
        album_items
            .into_iter()
            .filter(|x: &anyhow::Result<()>| x.is_err())
            .collect::<Vec<_>>()
    );

    let s = gpclient.media_items_stream();
    let media_items_results = s
        .map(|media_item_or| {
            let p = pool.clone();
            async move { fetch(&media_item_or?, &p, path).await }
        })
        .buffer_unordered(args.concurrency)
        .collect::<Vec<_>>()
        .await;
    println!(
        "{:?}",
        media_items_results
            .into_iter()
            .filter(|x| x.is_err())
            .collect::<Vec<_>>()
    );

    Ok(())
}
