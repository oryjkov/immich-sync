use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use colored::Colorize;
use dotenvy::dotenv;
use immich_api::apis::albums_api;
use immich_api::apis::configuration;
use immich_api::apis::configuration::Configuration;
use immich_api::apis::search_api;
use immich_api::models;
use serde::Deserialize;
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
    #[arg(long, default_value = "http://h4:2283/api")]
    immich_url: String,

    #[arg(long, default_value = "download/sqlite.db")]
    sqlite: String,

    #[arg(long, default_value = None)]
    album_gphoto_id: Option<String>,
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
    NotFound,              // Filename is not found in immich
    FoundMultiple, // Filename found multiple matches but none of the candidates had a matching metadata
    FoundUnique(String), // Filename found a single match but no matching metadata
    MatchedMultiple, // Metadata matched with multiple candidates
    MatchedUnique(String), // Metadata matched with exactly one candidate
}

async fn link_item(
    api_config: &Configuration,
    filename: &str,
    metadata: &str,
) -> Result<LookupResult> {
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
            // println!("{}", filename.yellow());
            // println!("{} {:?}", "gphoto metadata:".red(), gphoto_metadata);
            // println!("{} {:?}", "immich metadata:".green(), immich_metadata);
            // println!("gphoto metadata: {:?}", metadata);
            // println!("immich metadata: {:?}", immich_item);
        }
    }
    Ok(rv)
}

async fn link_album_items(
    pool: &Pool<Sqlite>,
    api_config: &Configuration,
    gphoto_album_id: &str,
) -> Result<Vec<(String, LookupResult)>> {
    let gphoto_items = sqlx::query(r#"SELECT id, filename, metadata FROM media_items where id IN (select media_item_id from album_items WHERE album_id = $1)"#)
        .bind(gphoto_album_id)
        .fetch_all(pool)
        .await?;

    let mut ress: HashMap<_, usize> = HashMap::new();
    let mut link_results = vec![];
    for gphoto_item in gphoto_items {
        let gphoto_id: &str = gphoto_item.try_get("id").unwrap();
        let filename: &str = gphoto_item.try_get("filename").unwrap();
        let metadata: &str = gphoto_item.try_get("metadata").unwrap();
        let res = link_item(api_config, filename, metadata).await?;
        let anon_res = match res {
            LookupResult::FoundUnique(_) => LookupResult::FoundUnique("_".to_string()),
            LookupResult::MatchedUnique(_) => LookupResult::MatchedUnique("_".to_string()),
            _ => res.clone(),
        };
        *ress.entry(anon_res).or_default() += 1;
        link_results.push((gphoto_id.to_string(), res));
    }
    println!("{:?}", ress);
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

async fn link_albums(
    pool: &Pool<Sqlite>,
    api_config: &Configuration,
    f: impl Fn(&str) -> bool,
) -> Result<()> {
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
    .filter(|(_, id)| f(id))
    .collect::<Vec<(String, String)>>();

    struct AlbumAlbumLink {
        gphoto_id: String,
        immich_id: String,
    }
    let mut links = vec![];
    for (name, gphoto_album_id) in gphoto_albums {
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
                    gphoto_id: gphoto_album_id,
                    immich_id: immich_id.clone(),
                });
            }
            None => {
                print!("{:?}: ", name);
                let linked_items = link_album_items(&pool, &api_config, &gphoto_album_id).await?;
                // let immich_album_id =
                //     create_linked_album(pool, api_config, &gphoto_album_id, &name).await?;
                // add_to_album(pool, api_config, &immich_album_id, &linked_items).await?;
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
async fn upload(
    pool: &Pool<Sqlite>,
    api_config: &Configuration,
    gphoto_item_id: &str,
) -> Result<()> {
    //
    // Download gphoto id
    // Upload to immich

    Ok(())
}

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
    let res = albums_api::create_album(api_config, req).await?;

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
    tx.commit()
        .await
        .with_context(|| format!("failed to create new immich album data in the db"))?;

    Ok(res.id)
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
    if let Some(id) = args.album_gphoto_id {
        link_albums(&pool, &api_config, |x| x == id).await?;
    }
    // link_albums(&pool, &api_config).await?;
    Ok(())
}
