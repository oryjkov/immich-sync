use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use colored::Colorize;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use dotenvy::dotenv;
use futures::StreamExt;
use gphotos_api::models::media_item;
use immich_api::apis::albums_api;
use immich_api::apis::assets_api;
use immich_api::apis::configuration;
use immich_api::apis::configuration::Configuration;
use immich_api::apis::search_api;
use immich_api::models;
use log::{debug, error, info, warn};
use serde::Deserialize;
use serde::Serialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
use std::env;
use std::mem::swap;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use unicode_normalization::UnicodeNormalization;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    immich_url: String,

    #[arg(long, default_value = "download/sqlite.db")]
    sqlite: String,

    #[arg(long, default_value = None)]
    gphoto_album_id: Option<String>,

    #[arg(long, default_value = None)]
    gphoto_item_id: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Default, Clone)]
#[serde(rename_all = "camelCase")]
struct VideoMetadata {
    camera_make: Option<String>,
    camera_model: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Default, Clone)]
#[serde(rename_all = "camelCase")]
struct PhotoMetadata {
    camera_make: Option<String>,
    camera_model: Option<String>,
    focal_length: Option<f64>,
    aperture_f_number: Option<f64>,
    iso_equivalent: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_exposure_time")]
    exposure_time: Option<u64>,
}
fn deserialize_exposure_time<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(exp_s) = s {
        let s = exp_s
            .trim_end_matches('s')
            .parse::<f64>()
            .map_err(serde::de::Error::custom)?;
        Ok(Some((s * 1e6).round() as u64))
    } else {
        Ok(None)
    }
}
fn deserialize_creation_time<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(date_str) = s {
        DateTime::parse_from_rfc3339(&date_str)
            .map(|dt| Some(dt.with_timezone(&Utc)))
            .map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}
#[derive(Debug, Deserialize, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
struct ImageData {
    #[serde(default, deserialize_with = "deserialize_creation_time")]
    creation_time: Option<DateTime<Utc>>,
    width: Option<String>,
    height: Option<String>,
    photo: Option<PhotoMetadata>,
    video: Option<VideoMetadata>,
}

impl From<models::AssetResponseDto> for ImageData {
    fn from(value: models::AssetResponseDto) -> ImageData {
        let exif = &value.exif_info;
        let exposure_time = exif
            .as_ref()
            .and_then(|exif| exif.exposure_time.clone().flatten())
            .map(|s| {
                let p = s.split('/').collect::<Vec<_>>();
                if p.len() == 0 {
                    (p[0].parse::<f64>().unwrap() * 1e6).round() as u64
                } else {
                    ((p[0].parse::<f64>().unwrap() / p[1].parse::<f64>().unwrap()) * 1e6).round()
                        as u64
                }
            });
        ImageData {
            creation_time: Some(
                DateTime::parse_from_rfc3339(&value.file_created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap(),
            ),
            width: exif.as_ref().and_then(|exif| {
                exif.exif_image_width
                    .flatten()
                    .map(|f| format!("{}", f as i64))
            }),
            height: exif.as_ref().and_then(|exif| {
                exif.exif_image_height
                    .flatten()
                    .map(|f| format!("{}", f as i64))
            }),
            photo: if value.r#type == models::AssetTypeEnum::Image {
                Some(PhotoMetadata {
                    camera_make: exif.as_ref().and_then(|exif| exif.make.clone().flatten()),
                    camera_model: exif.as_ref().and_then(|exif| exif.model.clone().flatten()),
                    aperture_f_number: exif.as_ref().and_then(|exif| exif.f_number.flatten()),
                    focal_length: exif.as_ref().and_then(|exif| exif.focal_length.flatten()),
                    iso_equivalent: exif
                        .as_ref()
                        .and_then(|exif| exif.iso.flatten().map(|x| x as u32)),
                    exposure_time,
                    ..Default::default()
                })
            } else {
                None
            },
            video: if value.r#type == models::AssetTypeEnum::Video {
                Some(VideoMetadata {
                    camera_make: exif.as_ref().and_then(|exif| exif.make.clone().flatten()),
                    camera_model: exif.as_ref().and_then(|exif| exif.model.clone().flatten()),
                })
            } else {
                None
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum LookupResult {
    NotFound,                // Filename is not found in immich
    FoundMultiple, // Filename found multiple matches but none of the candidates had a matching metadata
    FoundUnique(String), // Filename found a single match but no matching metadata
    MatchedMultiple, // Metadata matched with multiple candidates
    MatchedUnique(String), // Metadata matched with exactly one candidate
    MatchedUniqueDB(String), // Matched an item from the local db.
}

// Links a media item from google photos to a immich item. Linking is done by:
// 1. local DB mapping (for items that we have created),
// 2. filename and metadata.
async fn link_item(
    pool: &Pool<Sqlite>,
    api_config: &Configuration,
    gphoto_id: &str,
    filename: &str,
    metadata: &str,
) -> Result<LookupResult> {
    let local_match = sqlx::query(r#"SELECT immich_id FROM item_item_links WHERE gphoto_id = $1"#)
        .bind(gphoto_id)
        .fetch_optional(pool)
        .await?;
    if let Some(immich_id) = local_match {
        return Ok(LookupResult::MatchedUniqueDB(immich_id.get("immich_id")));
    }

    let gphoto_metadata: ImageData = serde_json::from_str(metadata)
        .with_context(|| format!("failed to parse gphoto metadata"))?;

    let search_req = models::MetadataSearchDto {
        original_file_name: Some(filename.to_string()),
        with_exif: Some(true),
        ..Default::default()
    };
    let mut rv = LookupResult::NotFound;
    let res = search_api::search_metadata(api_config, search_req).await?;
    if res.assets.items.len() == 1 {
        rv = LookupResult::FoundUnique(res.assets.items[0].id.clone());
    } else if res.assets.items.len() > 1 {
        rv = LookupResult::FoundMultiple;
    }
    for immich_item in &res.assets.items {
        let immich_metadata = ImageData::from(immich_item.clone());

        if match_metadata(&gphoto_metadata, &immich_metadata) {
            rv = match rv {
                LookupResult::MatchedUnique(_) => LookupResult::MatchedMultiple,
                _ => LookupResult::MatchedUnique(immich_item.id.clone()),
            };
        } else {
            debug!("{}: No metadata match!", filename.yellow());
            debug!("{} {:?}", "gphoto metadata:".red(), gphoto_metadata);
            debug!("{} {:?}", "immich metadata:".green(), immich_metadata);
            debug!("raw gphoto metadata: {:?}", metadata);
            debug!("raw immich metadata: {:?}", immich_item);
        }
    }
    Ok(rv)
}

#[derive(Debug)]
struct LinkedItem {
    gphoto_id: String,
    link_type: LookupResult,
}
// Tries to link all media items in gphoto album `gphoto_album_id` to immich media items.
// Returs the list of all "link" results, the return value has one element for each media
// item in the given gphoto album.
async fn link_album_items(
    pool: &Pool<Sqlite>,
    api_config: &Configuration,
    gphoto_client: &lib::GPClient,
    gphoto_album_id: &str,
    album_name: &str,
) -> Result<Vec<LinkedItem>> {
    let gphoto_items = gphoto_client
        .album_items_stream(gphoto_album_id)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|item| match item {
            Ok(item) => Some(item),
            Err(e) => {
                error!("failed listing items: {e:?}");
                None
            }
        })
        .collect::<Vec<_>>();
    // let gphoto_items = sqlx::query(r#"SELECT id, filename, metadata FROM media_items where id IN (select media_item_id from album_items WHERE album_id = $1)"#)
    //     .bind(gphoto_album_id)
    //     .fetch_all(pool)
    //     .await?;

    let mut ress: HashMap<_, usize> = HashMap::new();
    let mut link_results = vec![];
    for gphoto_item in gphoto_items {
        let gphoto_id = gphoto_item.id.unwrap();
        let filename = gphoto_item.filename.unwrap();
        let metadata = gphoto_item.media_metadata.unwrap();
        let metadata = serde_json::to_string(&metadata).unwrap();
        let res = link_item(pool, api_config, &gphoto_id, &filename, &metadata).await?;
        let anon_res = match res {
            LookupResult::FoundUnique(_) => LookupResult::FoundUnique("_".to_string()),
            LookupResult::MatchedUnique(_) => LookupResult::MatchedUnique("_".to_string()),
            _ => res.clone(),
        };
        *ress.entry(anon_res).or_default() += 1;
        link_results.push(LinkedItem {
            gphoto_id: gphoto_id.to_string(),
            link_type: res,
        });
    }
    info!("result from linking album items {album_name}: {:?}", ress);
    Ok(link_results)
}

fn match_metadata(gphoto_metadata: &ImageData, immich_metadata: &ImageData) -> bool {
    let mut gphoto_metadata = gphoto_metadata.clone();
    let mut immich_metadata = immich_metadata.clone();
    let mut immich_metadata_flipped = immich_metadata.clone();
    swap(
        &mut immich_metadata_flipped.width,
        &mut immich_metadata_flipped.height,
    );
    // Immich sometimes has empty strings for make/model.
    for m in vec![&mut gphoto_metadata, &mut immich_metadata] {
        m.photo.as_mut().map(|x| {
            if x.camera_make == Some("".to_string()) {
                x.camera_make = None
            }
        });
        m.photo.as_mut().map(|x| {
            if x.camera_model == Some("".to_string()) {
                x.camera_model = None
            }
        });
    }

    if gphoto_metadata.video.is_some() && immich_metadata.video.is_some() {
        // Immich has problems extracting some of the video metadata.
        gphoto_metadata.video = None;
        immich_metadata.video = None;
        return gphoto_metadata == immich_metadata || gphoto_metadata == immich_metadata_flipped;
    }

    if gphoto_metadata == immich_metadata || gphoto_metadata == immich_metadata_flipped {
        return true;
    } else {
        return false;
    }
}

// Goes through all of the albums in gphotos that pass the filter f and are not linked with
// an immich album and tries to link them. Linking is done based on the album name only.
// TODO: this picks a random albumm id for albums that have the same title. Detect it at least
async fn link_albums(
    api_config: &Configuration,
    gphoto_albums: impl Iterator<Item = (&str, &str)>, // List of (gphoto_album_ids, title)
) -> Result<HashMap<String, String>> {
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
    debug!("immich albums: {:?}", immich_albums);

    let mut links = HashMap::new();
    for (name, gphoto_album_id) in gphoto_albums {
        let nospace_name = name
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let name_nfc: String = name.nfc().collect();

        match m
            .get(name)
            .or_else(|| m.get(&nospace_name))
            .or_else(|| m.get(&name_nfc))
        {
            Some(immich_id) => {
                links.insert(gphoto_album_id.to_string(), immich_id.to_string());
            }
            None => {
                print!("{:?}: ", name);
                // let linked_items = link_album_items(&pool, &api_config, &gphoto_album_id).await?;
                // let immich_album_id =
                //     create_linked_album(pool, api_config, &gphoto_album_id, &name).await?;
                // add_to_album(pool, api_config, &immich_album_id, &linked_items).await?;
            }
        }
    }

    Ok(links)
}

// Saves the given albums links in the local DB.
async fn save_album_links(
    pool: &Pool<Sqlite>,
    links: impl Iterator<Item = (&String, &String)>,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    let mut cnt = 0;
    for (gphoto_id, immich_id) in links {
        sqlx::query(
            r#"
            INSERT INTO album_album_links (gphoto_id, immich_id) VALUES
                    ($1, $2)
            "#,
        )
        .bind(gphoto_id)
        .bind(immich_id)
        .execute(&mut *tx)
        .await?;
        cnt += 1;
    }
    tx.commit().await?;

    info!("linked {} albums", cnt);

    Ok(())
}

// Copies (downloads from gphoto and uploads to immich) all items given in `linked_items` and
// associates them with the immich album identified `immich_album_id`.
// TODO: this is WIP.
// TODO: which items to copy? NotFound only?
async fn copy_all_to_album(
    pool: &Pool<Sqlite>,
    api_config: &Configuration,
    gphoto_client: &lib::GPClient,
    immich_album_id: &str,
    linked_items: &[LinkedItem],
) -> Result<()> {
    let mut result = vec![];
    for linked_item in linked_items {
        let immich_id = match &linked_item.link_type {
            LookupResult::NotFound => {
                info!("Will copy item {:?}", linked_item);
                download_and_upload(pool, api_config, gphoto_client, &linked_item.gphoto_id).await?
            }
            // Re-do album association anyways since we are certain of the mapping here.
            LookupResult::MatchedUniqueDB(immich_id) => immich_id.clone(),
            _ => {
                debug!("Assuming item already exists in gphotos, skipping it");
                continue;
            }
        };
        result.push(uuid::Uuid::parse_str(&immich_id)?);
    }
    info!("uploaded {} items", result.len());
    if result.len() > 0 {
        let res = albums_api::add_assets_to_album(
            api_config,
            immich_album_id,
            models::BulkIdsDto { ids: result },
            None,
        )
        .await
        .with_context(|| format!("failed to add items to immich album {immich_album_id}"))?;
        debug!("add to album result: {res:?}");
    }
    Ok(())
}

// Downloads a media_item identified by `gphoto_id` from google photos and uploads it
// to immich. The newly created mapping (gphoto_id <=> immich_id) is stored in the local
// database.
async fn download_and_upload(
    pool: &Pool<Sqlite>,
    api_config: &Configuration,
    gphoto_client: &lib::GPClient,
    gphoto_item_id: &str,
) -> Result<String> {
    // Download gphoto id
    let (gphoto_item, bytes) = gphoto_client
        .fetch_media_item(gphoto_item_id)
        .await
        .with_context(|| format!("failed to fetch gphoto item id {}", gphoto_item_id))?;

    let creation_time = gphoto_item
        .media_metadata
        .as_ref()
        .unwrap()
        .creation_time
        .as_ref()
        .unwrap();

    let asset_data = reqwest::multipart::Part::bytes(bytes.to_vec()).file_name(
        gphoto_item
            .filename
            .unwrap_or(format!("no name on gphoto.name")),
    );

    let mut hasher = Sha1::new();
    hasher.input(&bytes.to_vec());

    // Upload to immich
    let res = assets_api::upload_asset(
        api_config,
        asset_data,
        &hasher.result_str(),
        "immich-sync",
        creation_time.clone(),
        creation_time.clone(),
        None,
        Some(&hasher.result_str()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .with_context(|| format!("upload_asset to immich failed"))?;
    info!("upload result: {:?}", res);
    sqlx::query(r#"INSERT INTO item_item_links (gphoto_id, immich_id) VALUES ($1, $2)"#)
        .bind(gphoto_item_id)
        .bind(&res.id)
        .execute(pool)
        .await
        .with_context(|| format!("failed to save item_item link to the db"))?;

    Ok(res.id)
}

// Creates an immich album named `title` that is then linked (in the local database) to
// a gphoto album identified by `gphoto_id`.
async fn create_linked_album(
    pool: &Pool<Sqlite>,
    api_config: &Configuration,
    gphoto_id: &str,
    title: &str,
) -> Result<String> {
    let req = models::CreateAlbumDto {
        album_name: title.to_string(),
        asset_ids: Some(vec![]),
        description: None,
        album_users: None, // When I passed in the current user, album page had 2 users registered
    };
    let res = albums_api::create_album(api_config, req)
        .await
        .with_context(|| format!("failed to create an immich album with title {title:?}"))?;

    let mut tx = pool.begin().await?;
    sqlx::query(r#"INSERT INTO created_albums (immich_id, creation_time) VALUES ($1, $2)"#)
        .bind(&res.id)
        .bind(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        )
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        r#"
            INSERT INTO album_album_links (gphoto_id, immich_id) VALUES
                    ($1, $2)
            "#,
    )
    .bind(gphoto_id)
    .bind(&res.id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await.with_context(|| {
        format!(
            "failed to write immich album data in the db for album {title:?} immich id: {}, gphoto id: {gphoto_id}",
            res.id
        )
    })?;

    Ok(res.id)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
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
    let gphoto_client = lib::GPClient::new_from_file("auth_token.json").await?;

    if let Some(gphoto_album_id) = args.gphoto_album_id {
        let album_metadata = gphoto_client
            .get_album(&gphoto_album_id)
            .await
            .with_context(|| format!("failed to get gphoto album with id {gphoto_album_id}"))?;

        let album_title: String = album_metadata.title.unwrap_or("<No title>".to_string());
        let immich_album_id: String = if let Some(immich_album_id) =
            sqlx::query(r#"SELECT immich_id FROM album_album_links WHERE gphoto_id = $1"#)
                .bind(&gphoto_album_id)
                .fetch_optional(&pool)
                .await?
                .map(|row| row.get("immich_id"))
        {
            info!(
                    "album {album_title:?} ({gphoto_album_id}) exists in immich and we already know it has immich id {immich_album_id}"
                );
            immich_album_id
        } else {
            let album_map = link_albums(
                &api_config,
                vec![(album_title.as_str(), gphoto_album_id.as_str())].into_iter(),
            )
            .await?;
            if let Some(immich_album_id) = album_map.get(&gphoto_album_id) {
                info!(
                    "album {album_title:?} ({gphoto_album_id}) found in immich and has id {immich_album_id}"
                );
                // Preserve the mapping in the local db (TODO: should do nothing if the mapping exists).
                save_album_links(&pool, album_map.iter()).await?;
                immich_album_id.clone()
            } else {
                // Create the new album in immich
                info!("album {album_title:?} ({gphoto_album_id}) does not exist in immich, creating it");
                create_linked_album(&pool, &api_config, &gphoto_album_id, &album_title).await?
            }
        };
        // Get the list of all media items in the gphoto album.
        let gphoto_items = link_album_items(
            &pool,
            &api_config,
            &gphoto_client,
            &gphoto_album_id,
            &album_title,
        )
        .await?;
        copy_all_to_album(
            &pool,
            &api_config,
            &gphoto_client,
            &immich_album_id,
            &gphoto_items,
        )
        .await?;
    }
    if let Some(gphoto_item_id) = args.gphoto_item_id {
        download_and_upload(&pool, &api_config, &gphoto_client, &gphoto_item_id).await?;
    }
    // link_albums(&pool, &api_config).await?;
    Ok(())
}
