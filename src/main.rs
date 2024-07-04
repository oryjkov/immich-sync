use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use colored::Colorize;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use futures::pin_mut;
use futures::stream;
use futures::StreamExt;
use gphotos_api::models::MediaItem;
use immich_api::apis::albums_api;
use immich_api::apis::assets_api;
use immich_api::apis::configuration;
use immich_api::apis::search_api;
use immich_api::models;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lib::coalescing_worker::CoalescingWorker;
use lib::gpclient::get_auth;
use lib::gpclient::GPClient;
use lib::immich_client::ImmichClient;
use lib::types::*;
use log::{debug, error, info, warn};
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;
use std::env;
use std::hash::Hash;
use std::mem::swap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use unicode_normalization::UnicodeNormalization;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    immich_url: String,

    #[arg(long, default_value = "sqlite.db")]
    db: String,

    #[arg(long, default_value = None)]
    gphoto_album_id: Option<String>,

    #[arg(long, default_value = None)]
    gphoto_item_id: Option<String>,

    #[arg(long, default_value = None)]
    all_shared: bool,

    #[arg(long)]
    client_secret: String,

    #[arg(long, default_value_t = 10)]
    download_concurrency: usize,

    #[arg(long, default_value_t = false)]
    read_only: bool,

    #[arg(long, default_value = None)]
    items: Option<usize>,

    #[arg(long, default_value_t = false)]
    early_exit: bool,

    #[arg(long, default_value = ".env")]
    immich_auth: String,

    #[arg(long, default_value = "auth_token.json")]
    auth_token: String,
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
                if p.len() == 1 {
                    (p[0].parse::<f64>().unwrap() * 1e6).round() as u64
                } else if p.len() == 2 {
                    ((p[0].parse::<f64>().unwrap() / p[1].parse::<f64>().unwrap()) * 1e6).round()
                        as u64
                } else {
                    panic!("strange input for exposure time: {:?}", s);
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
    NotFound,                      // Filename is not found in immich
    FoundMultiple, // Filename found multiple matches but none of the candidates had a matching metadata
    FoundUnique(ImmichItemId), // Filename found a single match but no matching metadata
    MatchedMultiple, // Metadata matched with multiple candidates
    MatchedUnique(ImmichItemId), // Metadata matched with exactly one candidate
    MatchedUniqueDB(ImmichItemId), // Matched an item from the local db.
}

// Links a media item from google photos to a immich item. Linking is done by:
// 1. local DB mapping (for items that we have created),
// 2. filename and metadata.
async fn link_item(
    pool: &Pool<Sqlite>,
    immich_client: &ImmichClient,
    gphoto_item: &MediaItem,
) -> Result<LookupResult> {
    let gphoto_id = GPhotoItemId(gphoto_item.id.as_ref().unwrap().clone());
    let filename = gphoto_item.filename.as_ref().unwrap();
    // TODO: get rid of this string intermediate conversion.
    let metadata = gphoto_item.media_metadata.as_ref().unwrap();
    let metadata = serde_json::to_string(&metadata).unwrap();

    let local_match = sqlx::query(r#"SELECT immich_id FROM item_item_links WHERE gphoto_id = $1"#)
        .bind(&gphoto_id.0)
        .fetch_optional(pool)
        .await?;
    if let Some(immich_id) = local_match {
        return Ok(LookupResult::MatchedUniqueDB(ImmichItemId(
            immich_id.get("immich_id"),
        )));
    }

    let gphoto_metadata: ImageData = serde_json::from_str(&metadata)
        .with_context(|| format!("failed to parse gphoto metadata"))?;

    let search_req = models::MetadataSearchDto {
        original_file_name: Some(filename.to_string()),
        with_exif: Some(true),
        ..Default::default()
    };
    let mut rv = LookupResult::NotFound;
    let res = search_api::search_metadata(&immich_client.get_config(), search_req).await?;
    if res.assets.items.len() == 1 {
        rv = LookupResult::FoundUnique(ImmichItemId(res.assets.items[0].id.clone()));
    } else if res.assets.items.len() > 1 {
        rv = LookupResult::FoundMultiple;
    }
    for immich_item in &res.assets.items {
        let immich_metadata = ImageData::from(immich_item.clone());

        if match_metadata(&gphoto_metadata, &immich_metadata) {
            rv = match rv {
                LookupResult::MatchedUnique(_) => LookupResult::MatchedMultiple,
                _ => LookupResult::MatchedUnique(ImmichItemId(immich_item.id.clone())),
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
    // gphoto_id: GPhotoItemId,
    gphoto_item: MediaItem,
    link_type: LookupResult,
}
// Tries to link all media items in gphoto album `gphoto_album_id` to immich media items.
// Returs the list of all "link" results, the return value has one element for each media
// item in the given gphoto album.
async fn link_album_items(
    pool: &Pool<Sqlite>,
    immich_client: &ImmichClient,
    gphoto_client: &GPClient,
    album_metadata: gphotos_api::models::Album,
) -> Result<Vec<LinkedItem>> {
    let gphoto_album_id = GPhotoAlbumId(album_metadata.id.ok_or(anyhow!("missing id"))?);
    let album_title: String = album_metadata.title.unwrap_or("<No title>".to_string());

    let gphoto_items = gphoto_client
        .album_items_stream(&gphoto_album_id)
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
    let link_results = stream::iter(gphoto_items.into_iter().map(|gphoto_item| async move {
        let id = gphoto_item.id.clone();
        Ok(LinkedItem {
            link_type: link_item(pool, immich_client, &gphoto_item)
                .await
                .with_context(|| format!("link item failed on {:?}", id))?,
            gphoto_item,
        })
    }))
    .buffer_unordered(1)
    .collect::<Vec<_>>()
    .await;

    let errors = link_results
        .iter()
        .filter(|x: &&Result<LinkedItem>| x.is_err())
        .collect::<Vec<_>>();
    if errors.len() > 0 {
        error!("link albums items errors: {:?}", errors);
    }
    let ok_res = link_results
        .into_iter()
        .filter_map(|r| match r {
            Ok(l) => Some(l),
            Err(_) => None,
        })
        .collect::<Vec<_>>();
    let ress = group_items(ok_res.iter());
    info!(
        "linking {album_title}({}): {:?}",
        album_metadata.product_url.unwrap_or("no_url!".to_string()),
        ress
    );

    Ok(ok_res)
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
// TODO: this picks a random album id for albums that have the same title. Detect it at least
async fn link_album(
    album_title: &str,
    immich_albums: &[(String, ImmichAlbumId)],
) -> Result<Option<ImmichAlbumId>> {
    // Maps various version of the (immich) album title to immich album id. The title "as-is" takes
    // precedence. We then lookup gphoto album title (variants) in that map.
    let mut m: HashMap<String, ImmichAlbumId> = HashMap::new();

    // Remove spaces - some albums have a trailing space.
    for (name, id) in immich_albums {
        let name = name
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        m.insert(name, id.clone());
    }

    // Unicode normalization. I had "Trip in Graubu\u{308}nden" and "Trip in Graubünden" in albums
    for (name, id) in immich_albums {
        let name: String = name.nfc().collect();
        m.insert(name, id.clone());
    }
    // This mapping takes precedence in case there are albums with trailing space and without.
    for (name, id) in immich_albums {
        m.insert(name.clone(), id.clone());
    }

    let nospace_name = album_title
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let name_nfc: String = album_title.nfc().collect();

    match m
        .get(album_title)
        .or_else(|| m.get(&nospace_name))
        .or_else(|| m.get(&name_nfc))
    {
        Some(immich_id) => Ok(Some(immich_id.clone())),
        None => Ok(None),
    }
}

// Saves the given albums links in the local DB.
async fn save_album_link(
    pool: &Pool<Sqlite>,
    gphoto_id: &GPhotoAlbumId,
    immich_id: &ImmichAlbumId,
) -> Result<bool> {
    let r = sqlx::query(
        r#"
        INSERT OR IGNORE INTO album_album_links (gphoto_id, immich_id) VALUES
                ($1, $2)
        "#,
    )
    .bind(&gphoto_id.0)
    .bind(&immich_id.0)
    .execute(pool)
    .await?;

    Ok(r.rows_affected() > 0)
}

// Copies (downloads from gphoto and uploads to immich) all items given in `linked_items` and
// associates them with the immich album identified `immich_album_id`.
// TODO: this is WIP.
// TODO: which items to copy? NotFound only?
async fn copy_all_to_album(
    immich_client: &ImmichClient,
    c: CoalescingWorker<WrappedMediaItem, ImmichItemId>,
    immich_album_id: &ImmichAlbumId,
    linked_items: &[LinkedItem],
    pb: ProgressBar,
) -> Result<()> {
    let mut work = vec![];
    for linked_item in linked_items {
        match &linked_item.link_type {
            LookupResult::NotFound => {
                info!("Will copy item {:?}", linked_item);
                work.push(linked_item.gphoto_item.clone());
            }
            _ => {
                debug!("Assuming item already exists in gphotos, skipping it");
                continue;
            }
        }
    }
    pb.inc((linked_items.len() - work.len()) as u64);
    let z = stream::iter(work)
        .map(|gphoto_item| {
            let pb = pb.clone();
            let c = c.clone();
            async move {
                let res = c.do_work(WrappedMediaItem(gphoto_item)).await;
                pb.inc(1);
                res
            }
        })
        .buffer_unordered(100) // 100 is just a large number,
        // concurrency is limited internally in the downloader.
        .collect::<Vec<_>>()
        .await;
    let mut result = z
        .into_iter()
        .filter_map(|r| match r {
            Ok(immich_id) => Some(immich_id),
            Err(e) => {
                error!("copy failed {:?}", e);
                None
            }
        })
        .collect::<Vec<_>>();
    debug!("uploaded {} items", result.len());
    for linked_item in linked_items {
        result.push(match &linked_item.link_type {
            // Re-do album association anyways since we are certain of the mapping here.
            LookupResult::MatchedUniqueDB(immich_id) => immich_id.clone(),
            _ => {
                continue;
            }
        });
    }
    let result = result
        .into_iter()
        .map(|id| uuid::Uuid::parse_str(&id.0).unwrap())
        .collect::<Vec<_>>();

    if result.len() > 0 {
        if immich_client.read_only {
            warn!("immich: add {:?} to album {}", result, immich_album_id.0);
        } else {
            let res = albums_api::add_assets_to_album(
                &immich_client.get_config(),
                &immich_album_id.0,
                models::BulkIdsDto { ids: result },
                None,
            )
            .await
            .with_context(|| format!("failed to add items to immich album {immich_album_id}"))?;
            debug!("add to album result: {res:?}");
        }
    }
    Ok(())
}

// Downloads a media_item identified by `gphoto_id` from google photos and uploads it
// to immich. The newly created mapping (gphoto_id <=> immich_id) is stored in the local
// database.
async fn download_and_upload(
    pool: &Pool<Sqlite>,
    immich_client: &ImmichClient,
    gphoto_client: &GPClient,
    gphoto_item: &MediaItem,
) -> Result<ImmichItemId> {
    // Download gphoto id
    let bytes = gphoto_client
        .fetch_media_item(gphoto_item)
        .await
        .with_context(|| {
            format!(
                "failed to fetch gphoto item id {}",
                gphoto_item.id.as_ref().unwrap()
            )
        })?;

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
            .clone()
            .unwrap_or(format!("no name on gphoto.name")),
    );

    let mut hasher = Sha1::new();
    hasher.input(&bytes.to_vec());

    // Upload to immich
    let res = assets_api::upload_asset(
        &(immich_client.get_config_for_writing()? as lib::immich_client::ApiConfigWrapper),
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
    debug!("upload result: {:?}", res);
    sqlx::query(r#"INSERT INTO item_item_links (gphoto_id, immich_id) VALUES ($1, $2)"#)
        .bind(&gphoto_item.id.as_ref().unwrap())
        .bind(&res.id)
        .execute(pool)
        .await
        .with_context(|| format!("failed to save item_item link to the db"))?;

    Ok(ImmichItemId(res.id))
}

// Creates an immich album named `title` that is then linked (in the local database) to
// a gphoto album identified by `gphoto_id`.
async fn create_linked_album(
    pool: &Pool<Sqlite>,
    immich_client: &ImmichClient,
    gphoto_id: &GPhotoAlbumId,
    title: &str,
) -> Result<ImmichAlbumId> {
    if immich_client.read_only {
        debug!("not creating immich album {title:?} when read-only");
        return Ok(ImmichAlbumId("dummy read=only album".to_string()));
    }
    let req = models::CreateAlbumDto {
        album_name: title.to_string(),
        asset_ids: Some(vec![]),
        description: None,
        album_users: None, // When I passed in the current user, album page had 2 users registered
    };
    let res = albums_api::create_album(
        &(immich_client.get_config_for_writing()? as lib::immich_client::ApiConfigWrapper),
        req,
    )
    .await
    .with_context(|| format!("failed to create an immich album with title {title:?}"))?;
    let immich_album_id = ImmichAlbumId(res.id);

    let mut tx = pool.begin().await?;
    sqlx::query(r#"INSERT INTO created_albums (immich_id, creation_time) VALUES ($1, $2)"#)
        .bind(&immich_album_id.0)
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
    .bind(&gphoto_id.0)
    .bind(&immich_album_id.0)
    .execute(&mut *tx)
    .await?;
    tx.commit().await.with_context(|| {
        format!(
            "failed to write immich album data in the db for album {title:?} immich id: {}, gphoto id: {gphoto_id}",
            immich_album_id
        )
    })?;

    Ok(immich_album_id)
}

async fn do_one_album(
    pool: &Pool<Sqlite>,
    immich_client: &ImmichClient,
    gphoto_client: &GPClient,
    cop: CoalescingWorker<WrappedMediaItem, ImmichItemId>,
    album_metadata: gphotos_api::models::Album,
    immich_albums: &[(String, ImmichAlbumId)],
    multi: MultiProgress,
) -> Result<Vec<LinkedItem>> {
    let gphoto_album_id = GPhotoAlbumId(album_metadata.id.clone().ok_or(anyhow!("missing id"))?);
    let pb = multi.add(ProgressBar::new(
        album_metadata
            .media_items_count
            .clone()
            .unwrap_or_default()
            .parse()
            .unwrap_or_default(),
    ));
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    let album_title: String = album_metadata
        .title
        .clone()
        .unwrap_or("<No title>".to_string());
    pb.set_message(format!("{:?}: finding immich album", album_title));

    let immich_album_id = if let Some(immich_album_id) =
        sqlx::query(r#"SELECT immich_id FROM album_album_links WHERE gphoto_id = $1"#)
            .bind(&gphoto_album_id.0)
            .fetch_optional(pool)
            .await?
            .map(|row| ImmichAlbumId(row.get("immich_id")))
    {
        debug!("album {album_title:?} ({gphoto_album_id}) exists in immich and we already know it has immich id {immich_album_id}");
        immich_album_id
    } else {
        if let Some(immich_album_id) = link_album(album_title.as_str(), &immich_albums).await? {
            debug!("album {album_title:?} ({gphoto_album_id}) found in immich and has id {immich_album_id}");
            // Preserve the mapping in the local db (TODO: should do nothing if the mapping exists).
            if save_album_link(pool, &gphoto_album_id, &immich_album_id).await? {
                immich_album_id
            } else {
                debug!("album {album_title:?} already exists in immich but is mapped to another album, creating a new one");
                create_linked_album(pool, immich_client, &gphoto_album_id, &album_title).await?
            }
        } else {
            // Create the new album in immich
            debug!(
                "album {album_title:?} ({gphoto_album_id}) does not exist in immich, creating it"
            );
            create_linked_album(pool, immich_client, &gphoto_album_id, &album_title).await?
        }
    };
    pb.set_message(format!("{:?}: linking album items", album_title));
    // Get the list of all media items in the gphoto album.
    let linked_items = link_album_items(pool, immich_client, gphoto_client, album_metadata).await?;
    if !immich_client.read_only {
        pb.set_message(format!("{:?}: copying album items", album_title));
        copy_all_to_album(&immich_client, cop, &immich_album_id, &linked_items, pb).await?;
    } else {
        debug!("skipping copy when read-only");
    }
    Ok(linked_items)
}

#[derive(PartialEq, Debug, Clone)]
struct WrappedMediaItem(MediaItem);
impl Eq for WrappedMediaItem {}
impl Hash for WrappedMediaItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.id.as_ref().unwrap().hash(state)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();
    let multi = MultiProgress::new();
    indicatif_log_bridge::LogWrapper::new(multi.clone(), logger)
        .try_init()
        .unwrap();
    let args = Args::parse();

    let _ = dotenvy::from_filename(args.immich_auth)
        .inspect_err(|err| warn!("failed to read .env file: {:?}", err));

    let mut create_schemas = false;
    if !std::path::Path::new(&args.db).exists() {
        warn!("DB not found, creating a new one in {}", args.db);
        let _ = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open(std::path::Path::new(&args.db))
            .with_context(|| format!("failed to create db file {}", args.db))?;
        create_schemas = true;
    }
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&args.db)
        .await
        .with_context(|| format!("failed to open db file {}", args.db))?;
    if create_schemas {
        let db_schema = include_str!("db_schema.sql");

        sqlx::raw_sql(db_schema)
            .execute(&pool)
            .await
            .with_context(|| format!("failed to create new db schema"))?;
    }

    let api_key = env::vars()
        .find(|(k, _)| k == "IMMICH_API_KEY")
        .map(|(_, v)| configuration::ApiKey {
            prefix: None,
            key: v,
        });
    let immich_client = ImmichClient::new(10, &args.immich_url, api_key, args.read_only);

    if !std::path::Path::new(&args.auth_token).exists() {
        warn!(
            "auth file {:?} does not exist, will request new auth",
            args.auth_token
        );
        get_auth(&args.client_secret, &args.auth_token).await?;
    }
    let gphoto_client = GPClient::new_from_file(&args.client_secret, &args.auth_token).await?;

    let cop = {
        let pool = pool.clone();
        let immich_client = immich_client.clone();
        let gphoto_client = gphoto_client.clone();
        let cop =
            CoalescingWorker::new(args.download_concurrency, move |item: WrappedMediaItem| {
                let pool = pool.clone();
                let immich_client = immich_client.clone();
                let gphoto_client = gphoto_client.clone();
                async move {
                    if immich_client.read_only {
                        // bail out early to not have to download items.
                        return Err(anyhow!("running read only, won't download"));
                    }
                    download_and_upload(&pool, &immich_client, &gphoto_client, &item.0)
                        .await
                        .with_context(|| format!("copy failed for item {}", item.0.id.unwrap()))
                }
            });
        cop
    };

    let res = albums_api::get_all_albums(&immich_client.get_config(), None, None)
        .await
        .with_context(|| format!("failed to get list of immich albums"))?;
    let immich_albums = res
        .into_iter()
        .map(|album| (album.album_name, ImmichAlbumId(album.id)))
        .collect::<Vec<_>>();
    let immich_albums = Arc::new(immich_albums);
    debug!("immich albums: {:?}", immich_albums);

    if let Some(gphoto_album_id) = args.gphoto_album_id {
        let gphoto_album_id = GPhotoAlbumId(gphoto_album_id);
        let album_metadata = gphoto_client
            .get_album(&gphoto_album_id)
            .await
            .with_context(|| format!("failed to get gphoto album with id {gphoto_album_id}"))?;
        let _ = do_one_album(
            &pool,
            &immich_client,
            &gphoto_client,
            cop.clone(),
            album_metadata,
            &immich_albums,
            multi.clone(),
        )
        .await?;
    }
    if let Some(gphoto_item_id) = args.gphoto_item_id {
        todo!("{}", gphoto_item_id);
        // let gphoto_item_id = GPhotoItemId(gphoto_item_id);
        // download_and_upload(&pool, &api_config, &gphoto_client, &gphoto_item_id).await?;
    }
    if args.all_shared {
        let all_albums_pb = multi.add(ProgressBar::new(0));
        all_albums_pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        all_albums_pb.set_message("Progress in albums");

        let mut shared_albums_results = vec![];
        let shared_albums_stream = gphoto_client.shared_albums_stream();
        pin_mut!(shared_albums_stream);
        while let Some(album_or) = shared_albums_stream.next().await {
            let album = album_or?;
            all_albums_pb.set_length(all_albums_pb.length().unwrap() + 1);
            debug!("copying album {:?}", album.title);
            let res = do_one_album(
                &pool,
                &immich_client,
                &gphoto_client,
                cop.clone(),
                album,
                &immich_albums,
                multi.clone(),
            )
            .await;
            all_albums_pb.inc(1);
            // Early exit if no NotFound items were encountered.
            if args.early_exit {
                if let Ok(link_items) = &res {
                    if link_items
                        .iter()
                        .filter(|x| match x.link_type {
                            LookupResult::NotFound => true,
                            _ => false,
                        })
                        .count()
                        == 0
                    {
                        info!("An album with no unseen items encountered, stopping");
                        break;
                    }
                }
            }
            shared_albums_results.push(res);
        }
        let errors = shared_albums_results
            .iter()
            .filter(|x| x.is_err())
            .collect::<Vec<_>>();
        if errors.len() > 0 {
            error!("shared albums errors: {:?}", errors);
        }
        let ok_res = shared_albums_results
            .into_iter()
            .filter_map(|r| match r {
                Ok(l) => Some(l),
                Err(_) => None,
            })
            .flatten()
            .collect::<Vec<_>>();
        let ress = group_items(ok_res.iter());
        info!("linking all shared albums items: {:?}", ress);
        for r in ok_res {
            match r.link_type {
                LookupResult::NotFound => {
                    info!(
                        "NotFound: {}, {}",
                        r.gphoto_item.filename.unwrap_or("no_filename>".to_string()),
                        r.gphoto_item.product_url.unwrap_or("no_url".to_string())
                    );
                }
                _ => {}
            }
        }
    }
    if let Some(n) = args.items {
        let items_pb = multi.add(ProgressBar::new(0));
        items_pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        items_pb.set_message("Listing all media items");

        let items_stream = gphoto_client
            .media_items_stream()
            .take(n)
            .map(|media_item| {
                let pb = items_pb.clone();
                pb.set_length(pb.length().unwrap() + 1);
                let pool = pool.clone();
                let immich_client = immich_client.clone();
                async move {
                    let item = media_item?;
                    let res = link_item(&pool, &immich_client, &item).await;
                    pb.inc(1);
                    res.map(|res| LinkedItem {
                        gphoto_item: item,
                        link_type: res,
                    })
                }
            })
            .collect::<Vec<_>>()
            .await;
        items_pb.set_message("linking items");
        let res = stream::iter(items_stream)
            .buffer_unordered(10)
            .collect::<Vec<_>>()
            .await;
        let num_err = res.iter().filter(|r| r.is_err()).count();
        info!("num errors: {}", num_err);

        let ress = group_items(res.iter().filter_map(|r| match r {
            Ok(l) => Some(l),
            Err(_) => None,
        }));
        info!("matching results: {:?}", ress);
    }
    Ok(())
}

fn group_items<'a>(items: impl Iterator<Item = &'a LinkedItem>) -> HashMap<LookupResult, usize> {
    let mut ress: HashMap<_, usize> = HashMap::new();
    items
        .map(|res| match res.link_type {
            LookupResult::FoundUnique(_) => {
                LookupResult::FoundUnique(ImmichItemId("_".to_string()))
            }
            LookupResult::MatchedUniqueDB(_) => {
                LookupResult::MatchedUniqueDB(ImmichItemId("_".to_string()))
            }
            LookupResult::MatchedUnique(_) => {
                LookupResult::MatchedUnique(ImmichItemId("_".to_string()))
            }
            _ => res.link_type.clone(),
        })
        .for_each(|anon_res| *ress.entry(anon_res).or_default() += 1);
    ress
}
