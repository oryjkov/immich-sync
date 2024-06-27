use anyhow::{anyhow, Context, Result};
use clap::Parser;
use dotenvy::dotenv;
use immich_api::apis::albums_api;
use immich_api::apis::assets_api;
use immich_api::apis::configuration;
use immich_api::apis::configuration::Configuration;
use immich_api::apis::search_api;
use immich_api::models;
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "http://h4:2283/api")]
    immich_url: String,

    #[arg(long, default_value = "download/sqlite.db")]
    sqlite: String,

    #[arg(long, default_value = "")]
    filename: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PhotoMetadata {
    camera_make: Option<String>,
    camera_model: Option<String>,
    focal_length: Option<f64>,
    aperture_f_number: Option<f64>,
    iso_equivalent: Option<u32>,
    exposure_time: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImageData {
    creation_time: Option<String>,
    width: Option<String>,
    height: Option<String>,
    photo: Option<PhotoMetadata>,
}

async fn link_items(pool: &Pool<Sqlite>, api_config: &Configuration) -> Result<()> {
    let gphoto_items = sqlx::query(r#"SELECT id, filename, metadata FROM media_items LIMIT 2"#)
        .fetch_all(pool)
        .await?;
    for gphoto_item in gphoto_items {
        let gphoto_id: String = gphoto_item.try_get("id").unwrap();
        let filename: String = gphoto_item.try_get("filename").unwrap();
        let gphoto_metadata: ImageData =
            serde_json::from_str(gphoto_item.try_get("metadata").unwrap())?;

        println!(
            "considering filename {filename}, (gphoto {gphoto_id}) with metadata\n{:?}",
            gphoto_metadata
        );

        let search_req = models::MetadataSearchDto {
            original_file_name: Some(filename),
            ..Default::default()
        };
        let mut res = search_api::search_metadata(api_config, search_req).await?;
        println!("found {} immich asset(s)", res.assets.items.len());
        if res.assets.items.len() == 1 {
            let immich_item = res.assets.items.pop().unwrap();
            let immich_item = assets_api::get_asset_info(api_config, &immich_item.id, None).await?;

            let exif = &immich_item.exif_info;
            let immich_metadata = ImageData {
                creation_time: Some(immich_item.file_created_at),
                width: exif
                    .as_ref()
                    .map(|exif| {
                        exif.exif_image_width
                            .flatten()
                            .map(|f| format!("{}", f as i64))
                    })
                    .flatten(),
                height: exif
                    .as_ref()
                    .map(|exif| {
                        exif.exif_image_height
                            .flatten()
                            .map(|f| format!("{}", f as i64))
                    })
                    .flatten(),
                photo: None,
                // width: exif.map(|exif| {
                //     exif.exif_image_width
                //         .flatten()
                //         .map(|f| format!("{}", f as i64))
                // }),
            };
            println!("immich asset id {}", immich_item.id);
            println!("gphoto metadata: {:?}", gphoto_metadata);
            println!("immich metadata: {:?}", immich_metadata);
        }
    }
    Ok(())
}

async fn link_albums(pool: &Pool<Sqlite>, api_config: &Configuration) -> Result<()> {
    let res = albums_api::get_all_albums(&api_config, None, None).await?;
    let immich_albums = res
        .into_iter()
        .map(|album| (album.album_name, album.id))
        .collect::<Vec<_>>();

    // Maps various version of the (immich) album title to immich album id. The title "as-is" takes
    // precedence. We then lookup gphoto album title (variants) in that map.
    let mut m: HashMap<String, String> = HashMap::new();

    // Remove spaces - some albums have a trailing space.
    for (name, id) in &immich_albums {
        let name = name
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        m.insert(name, id.clone());
    }

    // Unicode normalization. I had "Trip in Graubu\u{308}nden" and "Trip in Graubünden" in albums
    for (name, id) in &immich_albums {
        let name: String = name.nfc().collect();
        m.insert(name, id.clone());
    }
    // This mapping takes precedence in case there are albums with trailing space and without.
    for (name, id) in &immich_albums {
        m.insert(name.clone(), id.clone());
    }

    let gphoto_albums = sqlx::query(
        r#"SELECT id, title FROM albums WHERE id NOT IN (SELECT gphoto_id FROM album_album_links)"#,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| (row.try_get("title").unwrap(), row.try_get("id").unwrap()))
    .filter(|(title, _): &(String, String)| !title.is_empty() && title != "Untitled")
    .collect::<Vec<(String, String)>>();

    struct AlbumAlbumLink {
        gphoto_id: String,
        immich_id: String,
    }
    let mut links = vec![];
    for (name, id) in gphoto_albums {
        let nospace_name = name
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let name_nfc: String = name.nfc().collect();

        match m
            .get(&name)
            .or_else(|| m.get(&nospace_name))
            .or_else(|| m.get(&name_nfc))
        {
            Some(immich_id) => {
                links.push(AlbumAlbumLink {
                    gphoto_id: id,
                    immich_id: immich_id.clone(),
                });
            }
            None => {
                println!("not found {:?}", name);
            }
        }
    }

    let mut tx = pool.begin().await?;
    for link in &links {
        sqlx::query(
            r#"
            INSERT INTO album_album_links (gphoto_id, immich_id) VALUES
                    ($1, $2)
            "#,
        )
        .bind(&link.gphoto_id)
        .bind(&link.immich_id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    println!("linked {} albums", links.len());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file not found");
    let args = Args::parse();
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&args.sqlite)
        .await?;
    let api_config = configuration::Configuration {
        api_key: env::vars()
            .find(|(k, _)| k == "API_KEY")
            .map(|(_, v)| configuration::ApiKey {
                prefix: None,
                key: v,
            }),
        base_path: args.immich_url,
        ..Default::default()
    };
    // link_items(&pool, &api_config).await?;
    link_albums(&pool, &api_config).await?;

    Ok(())
}
