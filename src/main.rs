use anyhow::{anyhow, Context, Result};
use clap::{ArgAction, Parser};
use colored::Colorize;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use futures::pin_mut;
use futures::stream::{self, StreamExt};
use gphotos_api::models::{Album, MediaItem};
use immich_api::apis::albums_api;
use immich_api::apis::assets_api;
use immich_api::apis::configuration;
use immich_api::apis::search_api;
use immich_api::models;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use lib::gpclient::get_auth;
use lib::gpclient::GPClient;
use lib::immich_client::ImmichClient;
use lib::match_metadata::{compare_metadata, ImageData};
use lib::types::*;
use log::Level::{Debug, Warn};
use log::{debug, error, info, log_enabled, warn};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Row, Sqlite};
use std::collections::{HashMap, HashSet};
use std::env;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use unicode_normalization::UnicodeNormalization;

/// Import google photo data into Immich.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Immich API url, should normally include "/api" at the end.
    #[arg(long)]
    immich_url: String,

    /// Local database, should be persisted between runs. Back it up with the rest of immich data.
    #[arg(long, default_value = "sqlite.db")]
    db: String,

    /// Id of the google photo album to sync.
    #[arg(long, default_value = None)]
    gphoto_album_id: Option<String>,

    // #[arg(long, value_parser = parse_opt_usize)]
    /// Set to process all shared gphoto albums that the user is part of. If a value is given, then
    /// it is interpreted as usize limiting num of shared albums processed.
    #[arg(long, value_name = "shared_albums", action = ArgAction::Set)]
    shared_albums: Option<Option<String>>,
    /// Goes together with --shared-albums. If set, will exit as soon as an album with no unseen items
    /// is encountered.
    #[arg(long, default_value_t = false)]
    early_exit: bool,

    /// Google Photo API client ID.
    #[arg(long, default_value = "client-secret.json")]
    client_secret: String,

    /// Google photo API token. Will be created if does not exist. Creation requires user
    /// interaction via a local web server that runs on http://localhost:8080.
    #[arg(long, default_value = "auth_token.json")]
    auth_token: String,

    /// Max media items to download from gphoto concurrently.
    #[arg(long, default_value_t = 10)]
    download_concurrency: usize,

    /// Do not make any changes to Immich or the local db.
    #[arg(long, default_value_t = false)]
    read_only: bool,

    /// If set, will list up to this many media items from google photos and import them.
    #[arg(long, default_value = None)]
    items: Option<usize>,

    // File with the Immich API token.
    #[arg(long, default_value = ".env")]
    immich_auth: String,
}

lazy_static! {
    static ref STATS: Arc<Mutex<HashMap<&'static str, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));
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

#[derive(Debug, Default)]
struct ScanResult {
    media_items: HashMap<GPhotoItemId, MediaItem>,
    albums: HashMap<GPhotoAlbumId, Album>,
    associations: HashMap<GPhotoAlbumId, HashSet<GPhotoItemId>>,
}
#[derive(Debug, Default)]
struct SearchResult {
    media_items: HashMap<GPhotoItemId, ElementLinkResult<ImmichItemId>>,
    albums: HashMap<GPhotoAlbumId, ElementLinkResult<ImmichAlbumId>>,
}
impl SearchResult {
    fn log_summary(&self) {
        let mut items_summary: HashMap<String, usize> = HashMap::new();
        for (_, e) in &self.media_items {
            let group = match e {
                ElementLinkResult::Unknown(_) => "item skipped - no good match",
                ElementLinkResult::Found(_) => "found metadata",
                ElementLinkResult::ExistsInDB(_) => "found db",
                ElementLinkResult::CreateNew => "copy to immich",
            };
            *items_summary.entry(group.to_string()).or_default() += 1;
        }
        let mut albums_summary: HashMap<String, usize> = HashMap::new();
        for (_, e) in &self.albums {
            let group = match e {
                ElementLinkResult::Unknown(_) => "skipped",
                ElementLinkResult::Found(_) => "found metadata",
                ElementLinkResult::ExistsInDB(_) => "found db",
                ElementLinkResult::CreateNew => "create new",
            };
            *albums_summary.entry(group.to_string()).or_default() += 1;
        }
        info!(
            "search results: items: {:?}, albums: {:?}",
            items_summary, albums_summary
        );
    }
}

#[derive(Debug)]
enum ElementLinkResult<LinkedType> {
    ExistsInDB(LinkedType), // Element found in the db
    Found(LinkedType),      // Element found based on metadata, should record in the db
    CreateNew,              // Element not found, should create it
    Unknown(String),        // IDK!
}

async fn scan_one_album(
    pool: &Pool<Sqlite>,
    gphoto_client: &GPClient,
    gphoto_album_id: GPhotoAlbumId,
    album: Album,
    result: &mut ScanResult,
) -> Result<bool> {
    let album_items = gphoto_client
        .album_items_stream(&gphoto_album_id)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|item| match item {
            Ok(item) => Some((GPhotoItemId(item.id.clone().unwrap()), item)),
            Err(e) => {
                error!("failed listing items: {e:?}");
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let mut new_items = false;
    for (gphoto_id, _) in &album_items {
        if sqlx::query(r#"SELECT immich_id FROM item_item_links WHERE gphoto_id = $1"#)
            .bind(&gphoto_id.0)
            .fetch_optional(pool)
            .await?
            .is_some()
        {
            new_items = true;
            break;
        }
    }
    result.albums.insert(gphoto_album_id.clone(), album);
    result.associations.insert(
        gphoto_album_id.clone(),
        album_items.iter().map(|(k, _)| k.clone()).collect(),
    );
    result.media_items.extend(album_items);
    Ok(new_items)
}

async fn scan(
    pool: &Pool<Sqlite>,
    args: &Args,
    multi: &MultiProgress,
    gphoto_client: &GPClient,
) -> Result<ScanResult> {
    let mut result = ScanResult::default();
    // Go through gphoto API and pick what we're looking for.
    let num_shared = match args.shared_albums.as_ref() {
        Some(Some(value)) => value.parse::<usize>().ok(),
        Some(None) => Some(usize::MAX),
        None => None,
    };

    if let Some(gphoto_album_id) = args.gphoto_album_id.as_ref() {
        let gphoto_album_id = GPhotoAlbumId(gphoto_album_id.clone());
        let album_metadata = gphoto_client
            .get_album(&gphoto_album_id)
            .await
            .with_context(|| format!("failed to get gphoto album with id {gphoto_album_id}"))?;
        scan_one_album(
            pool,
            gphoto_client,
            gphoto_album_id,
            album_metadata,
            &mut result,
        )
        .await?;
    }
    if let Some(mut num_shared) = num_shared {
        let all_albums_pb = multi.add(ProgressBar::new(if num_shared == usize::MAX {
            0
        } else {
            num_shared as u64
        }));
        all_albums_pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        all_albums_pb.set_message("Scanning gphoto albums");

        let shared_albums_stream = gphoto_client.shared_albums_stream();
        pin_mut!(shared_albums_stream);
        while let Some(album_or) = shared_albums_stream.next().await {
            let album = album_or?;
            let gphoto_album_id = GPhotoAlbumId(album.id.clone().unwrap());
            let new_items =
                scan_one_album(pool, gphoto_client, gphoto_album_id, album, &mut result).await?;

            if num_shared == usize::MAX {
                all_albums_pb.set_length(all_albums_pb.length().unwrap() + 1);
            }

            all_albums_pb.inc(1);
            num_shared -= 1;
            if num_shared == 0 || (args.early_exit && !new_items) {
                break;
            }
        }
    }
    if let Some(mut n) = args.items {
        let items_pb = multi.add(ProgressBar::new(0));
        items_pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        items_pb.set_message("Listing all media items");

        let s = gphoto_client.media_items_stream();
        pin_mut!(s);
        while let Some(media_item) = s.next().await {
            let media_item = media_item?;
            result
                .media_items
                .insert(GPhotoItemId(media_item.id.clone().unwrap()), media_item);
            n -= 1;
            if n == 0 {
                break;
            }
        }
    }

    Ok(result)
}
async fn search(
    multi: &MultiProgress,
    scan_result: &ScanResult,
    pool: &Pool<Sqlite>,
    immich_client: &ImmichClient,
) -> Result<SearchResult> {
    let media_items_pb = multi.add(ProgressBar::new(scan_result.media_items.len() as u64));
    media_items_pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    media_items_pb.set_message("Linking media items");

    let mut result = SearchResult::default();
    // Find what we can in immich/local db and establish links. What can't be found will be either
    // skipped or created (in the stage that follows)
    result.media_items = stream::iter(scan_result.media_items.iter().map(
        |(gphoto_id, media_item)| {
            let pb = media_items_pb.clone();
            async move {
                let r = (gphoto_id, link_item(pool, immich_client, media_item).await);
                pb.inc(1);
                r
            }
        },
    ))
    .buffer_unordered(10)
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .filter_map(|(gphoto_id, link_res)| match link_res {
        Ok(res) => Some((gphoto_id, res)),
        Err(e) => {
            error!("lookup failed: {:?}", e);
            None
        }
    })
    .map(|(gphoto_id, (link_res, message))| {
        let x = match link_res {
            LookupResult::MatchedUniqueDB(immich_id) => ElementLinkResult::ExistsInDB(immich_id),
            LookupResult::MatchedUnique(immich_id) => ElementLinkResult::Found(immich_id),
            LookupResult::NotFound => ElementLinkResult::CreateNew,
            _ => ElementLinkResult::Unknown(message),
        };
        (gphoto_id.clone(), x)
    })
    .collect();

    let albums_pb = multi.add(ProgressBar::new(scan_result.albums.len() as u64));
    albums_pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    albums_pb.set_message("Linking albums");

    let immich_albums = get_immich_albums(&immich_client).await?;
    for (gphoto_album_id, gphoto_album) in &scan_result.albums {
        let x = link_album(pool, gphoto_album, &immich_albums).await?;
        albums_pb.inc(1);
        result.albums.insert(gphoto_album_id.clone(), x);
    }
    Ok(result)
}

async fn write(
    multi: &MultiProgress,
    search_result: &SearchResult,
    scan_result: &ScanResult,
    pool: &Pool<Sqlite>,
    immich_client: &ImmichClient,
    gphoto_client: &GPClient, // needed for downloading photos
) -> Result<()> {
    let mut linked_albums = HashMap::new();
    for (gphoto_id, link) in &search_result.albums {
        match link {
            ElementLinkResult::ExistsInDB(immich_id) => {
                linked_albums.insert(gphoto_id.clone(), immich_id.clone());
            }
            ElementLinkResult::Found(immich_id) => {
                if immich_client.read_only {
                    info!("will write album link {} <-> {}", gphoto_id, immich_id);
                } else {
                    save_album_link(pool, gphoto_id, immich_id).await?;
                }
                linked_albums.insert(gphoto_id.clone(), immich_id.clone());
            }
            ElementLinkResult::CreateNew => {
                let album_metadata = scan_result.albums.get(gphoto_id).unwrap();
                if immich_client.read_only {
                    info!("will have created album titled {:?}", album_metadata.title);
                    linked_albums.insert(
                        gphoto_id.clone(),
                        ImmichAlbumId(format!(
                            "NEW_ALBUM:{}",
                            album_metadata.product_url.clone().unwrap_or_default()
                        )),
                    );
                } else {
                    let immich_id = create_linked_album(
                        pool,
                        immich_client,
                        gphoto_id,
                        album_metadata.title.as_ref().unwrap(),
                    )
                    .await?;
                    linked_albums.insert(gphoto_id.clone(), immich_id);
                }
            }
            ElementLinkResult::Unknown(_) => {
                error!("should not happen for albums");
            }
        }
    }

    let items_copy_pb = multi.add(ProgressBar::new(
        search_result
            .media_items
            .iter()
            .filter(|(_, x)| match x {
                ElementLinkResult::CreateNew => true,
                _ => false,
            })
            .count() as u64,
    ));
    items_copy_pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    items_copy_pb.set_message("Copying media items");

    // Goes through media_items and performs all the actions to sync them to immich. As a result
    // builds a map from GPhotoItemId to ImmichItemId (either new or existing).
    let linked_items: HashMap<GPhotoItemId, ImmichItemId> =
        stream::iter(search_result.media_items.iter().map(|(gphoto_id, link)| {
            let pb = items_copy_pb.clone();
            async move {
                match link {
                    ElementLinkResult::ExistsInDB(immich_id) => Some(immich_id.clone()),
                    ElementLinkResult::Found(immich_id) => {
                        if immich_client.read_only {
                            info!("will write item link {} <-> {}", gphoto_id, immich_id);
                        } else {
                            let add_res = sqlx::query(
                                r#"
INSERT INTO item_item_links (gphoto_id, immich_id, link_type, insert_time)
VALUES ($1, $2, $3, $4)"#,
                            )
                            .bind(&gphoto_id.0)
                            .bind(&immich_id.0)
                            .bind("MatchedUnique")
                            .bind(
                                SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs() as i64,
                            )
                            .execute(pool)
                            .await;

                            if add_res.is_err() {
                                error!("failed to add the link {} {} to db", gphoto_id, immich_id);
                                return None;
                            }
                        }
                        Some(immich_id.clone())
                    }
                    ElementLinkResult::CreateNew => {
                        let r = if immich_client.read_only {
                            info!("will copy {} to immich", gphoto_id);
                            Some(ImmichItemId("NEW_ITEM".to_string()))
                        } else {
                            download_and_upload(
                                pool,
                                immich_client,
                                gphoto_client,
                                scan_result.media_items.get(gphoto_id).unwrap(),
                            )
                            .await
                            .ok()
                        };
                        pb.inc(1);
                        r
                    }
                    ElementLinkResult::Unknown(message) => {
                        if log_enabled!(Warn) {
                            let metadata = scan_result.media_items.get(gphoto_id).unwrap();
                            warn!(
                                "don't know what to do with {} {}",
                                metadata.filename.clone().unwrap_or_default(),
                                metadata.product_url.clone().unwrap_or_default()
                            );
                            if !message.is_empty() {
                                warn!("debug message: {}", message);
                            }
                        }
                        None
                    }
                }
                .map(|l| (gphoto_id.clone(), l))
            }
        }))
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|x| x)
        .collect();

    let immich_associations: HashMap<_, _> = scan_result
        .associations
        .iter()
        .filter_map(|(gphoto_album_id, gphoto_items)| {
            let immich_album_id = linked_albums.get(gphoto_album_id)?;
            let immich_items: HashSet<_> = gphoto_items
                .iter()
                .filter_map(|gphoto_item_id| linked_items.get(gphoto_item_id))
                .collect();
            Some((immich_album_id, immich_items))
        })
        .collect();

    let albums_add_pb = multi.add(ProgressBar::new(search_result.albums.len() as u64));
    albums_add_pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    albums_add_pb.set_message("Adding media items to albums");

    // Associate all the items with corresponding immich albums.
    stream::iter(
        immich_associations
            .into_iter()
            .map(|(immich_album_id, immich_items)| {
                let pb = albums_add_pb.clone();
                async move {
                    if immich_client.read_only {
                        info!(
                            "will add {} items to immich album {}",
                            immich_items.len(),
                            immich_album_id,
                        );
                    } else {
                        let immich_ids: Vec<_> = immich_items
                            .iter()
                            .map(|id| {
                                uuid::Uuid::parse_str(&id.0)
                                    .with_context(|| format!("while parsing {}", id.0))
                                    .unwrap()
                            })
                            .collect();

                        let _ = albums_api::add_assets_to_album(
                            &(immich_client.get_config_for_writing().unwrap()
                                as lib::immich_client::ApiConfigWrapper),
                            &immich_album_id.0,
                            models::BulkIdsDto { ids: immich_ids },
                            None,
                        )
                        .await
                        .map_err(|e| {
                            error!(
                                "failed to add items to immich album {immich_album_id}: {:?}",
                                e
                            )
                        });
                        pb.inc(1);
                    }
                }
            }),
    )
    .buffer_unordered(1)
    .collect::<Vec<_>>()
    .await;
    Ok(())
}

// Links a media item from google photos to a immich item. Linking is done by:
// 1. local DB mapping (for items that we have created),
// 2. filename and metadata.
async fn link_item(
    pool: &Pool<Sqlite>,
    immich_client: &ImmichClient,
    gphoto_item: &MediaItem,
) -> Result<(LookupResult, String)> {
    let gphoto_id = GPhotoItemId(gphoto_item.id.as_ref().unwrap().clone());
    let filename = gphoto_item.filename.as_ref().unwrap();
    let mut message = "".to_string();

    let local_match = sqlx::query(r#"SELECT immich_id FROM item_item_links WHERE gphoto_id = $1"#)
        .bind(&gphoto_id.0)
        .fetch_optional(pool)
        .await?;
    if let Some(immich_id) = local_match {
        return Ok((
            LookupResult::MatchedUniqueDB(ImmichItemId(immich_id.get("immich_id"))),
            message,
        ));
    }

    let gphoto_metadata: ImageData = gphoto_item
        .media_metadata
        .as_ref()
        .ok_or(anyhow!("missing metadata"))?
        .as_ref()
        .try_into()
        .with_context(|| {
            format!(
                "while converting {}",
                serde_json::to_string(gphoto_item.media_metadata.as_ref().unwrap()).unwrap()
            )
        })?;

    let search_req = models::MetadataSearchDto {
        original_file_name: Some(filename.to_string()),
        with_exif: Some(true),
        ..Default::default()
    };
    let mut rv = LookupResult::NotFound;
    let res = search_api::search_metadata(&immich_client.get_config(), search_req).await?;
    (*STATS.lock().unwrap().entry("item_searched").or_default()) += 1;
    if res.assets.items.len() == 1 {
        rv = LookupResult::FoundUnique(ImmichItemId(res.assets.items[0].id.clone()));
    } else if res.assets.items.len() > 1 {
        rv = LookupResult::FoundMultiple;
    }
    for immich_item in &res.assets.items {
        let immich_metadata = ImageData::from(immich_item.clone());

        if compare_metadata(&gphoto_metadata, &immich_metadata) {
            rv = match rv {
                LookupResult::MatchedUnique(_) => LookupResult::MatchedMultiple,
                _ => LookupResult::MatchedUnique(ImmichItemId(immich_item.id.clone())),
            };
        } else {
            if log_enabled!(Debug) {
                message.push_str(&format!(
                    "{}: No metadata match! gphoto_id: {}\n",
                    filename.yellow(),
                    gphoto_id
                ));
                message.push_str(&format!(
                    "{} {:?}\n{} {:?}\n",
                    "gphoto metadata:".red(),
                    gphoto_metadata,
                    "immich metadata:".green(),
                    immich_metadata
                ));
                message.push_str(&format!(
                    "raw gphoto metadata: {}\nraw immich metadata: {}",
                    serde_json::to_string(gphoto_item.media_metadata.as_ref().unwrap()).unwrap(),
                    serde_json::to_string(&immich_item).unwrap()
                ));
            }
        }
    }
    Ok((rv, message))
}

// Goes through all of the albums in gphotos that pass the filter f and are not linked with
// an immich album and tries to link them. Linking is done based on the album name only.
// TODO: this picks a random album id for albums that have the same title. Detect it at least
async fn link_album(
    pool: &Pool<Sqlite>,
    album_metadata: &gphotos_api::models::Album,
    immich_albums: &HashMap<String, Vec<ImmichAlbumId>>,
) -> Result<ElementLinkResult<ImmichAlbumId>> {
    let gphoto_album_id = GPhotoAlbumId(album_metadata.id.clone().ok_or(anyhow!("missing id"))?);
    let album_title: String = album_metadata
        .title
        .clone()
        .unwrap_or("<No title>".to_string());

    if let Some(immich_album_id) =
        sqlx::query(r#"SELECT immich_id FROM album_album_links WHERE gphoto_id = $1"#)
            .bind(&gphoto_album_id.0)
            .fetch_optional(pool)
            .await?
            .map(|row| ImmichAlbumId(row.get("immich_id")))
    {
        debug!("album {album_title:?} ({gphoto_album_id}) exists in immich and we already know it has immich id {immich_album_id}");
        return Ok(ElementLinkResult::ExistsInDB(immich_album_id));
    };

    if let Some(immich_album_id) = {
        let nospace_name = album_title
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let name_nfc: String = album_title.nfc().collect();
        match immich_albums
            .get(&album_title)
            .or_else(|| immich_albums.get(&nospace_name))
            .or_else(|| immich_albums.get(&name_nfc))
        {
            Some(immich_ids) => {
                if immich_ids.len() == 0 {
                    panic!("");
                } else if immich_ids.len() == 1 {
                    Some(immich_ids[0].clone())
                } else {
                    None
                }
            }
            None => None,
        }
    } {
        debug!(
            "album {album_title:?} ({gphoto_album_id}) matched with immich album {immich_album_id}"
        );
        if let Some(_) = sqlx::query(
            r#"SELECT gphoto_album_id FROM album_album_links WHERE immich_album_id = $1"#,
        )
        .bind(&immich_album_id.0)
        .fetch_optional(pool)
        .await?
        .map(|row| ImmichAlbumId(row.get("immich_id")))
        {
            debug!("album titled {album_title:?} already exists in immich but is mapped to another album, creating a new one");
            Ok(ElementLinkResult::CreateNew)
        } else {
            // Preserve the mapping in the local db.
            save_album_link(pool, &gphoto_album_id, &immich_album_id).await?;
            Ok(ElementLinkResult::Found(immich_album_id))
        }
    } else {
        // Create the new album in immich
        debug!("album {album_title:?} ({gphoto_album_id}) does not exist in immich, creating it");
        Ok(ElementLinkResult::CreateNew)
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
    (*STATS.lock().unwrap().entry("items_uploaded").or_default()) += 1;
    debug!("upload result: {:?}", res);
    sqlx::query(r#"INSERT INTO item_item_links (gphoto_id, immich_id, link_type, insert_time) VALUES ($1, $2, $3, $4)"#)
        .bind(&gphoto_item.id.as_ref().unwrap())
        .bind(&res.id)
        .bind("MatchedUniqueDB")
        .bind(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        )
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
    (*STATS.lock().unwrap().entry("albums_created").or_default()) += 1;
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
            INSERT INTO album_album_links (gphoto_id, immich_id, insert_time) VALUES
                    ($1, $2, $3)
            "#,
    )
    .bind(&gphoto_id.0)
    .bind(&immich_album_id.0)
    .bind(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    )
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

async fn check_and_update_schema(pool: &Pool<Sqlite>) -> Result<()> {
    let r = sqlx::query(r"SELECT insert_time FROM item_item_links LIMIT 1")
        .fetch_optional(pool)
        .await;

    if r.is_err() {
        warn!("need to update the db schema");
        let update_schema = r#"
ALTER TABLE "album_album_links" ADD COLUMN insert_time INTEGER DEFAULT NULL;
UPDATE "album_album_links" SET insert_time = unixepoch(CURRENT_TIMESTAMP);

ALTER TABLE "item_item_links" ADD COLUMN link_type TEXT DEFAULT NULL;
UPDATE "item_item_links" SET link_type = "MatchedUniqueDB";

ALTER TABLE "item_item_links" ADD COLUMN insert_time INTEGER DEFAULT NULL;
UPDATE "item_item_links" SET insert_time = unixepoch(CURRENT_TIMESTAMP);
"#;

        sqlx::raw_sql(update_schema)
            .execute(pool)
            .await
            .with_context(|| format!("failed to update the new db schema. oops"))?;
    }
    Ok(())
}

async fn get_immich_albums(
    immich_client: &ImmichClient,
) -> Result<HashMap<String, Vec<ImmichAlbumId>>> {
    let res = albums_api::get_all_albums(&immich_client.get_config(), None, None)
        .await
        .with_context(|| format!("failed to get list of immich albums"))?;

    let immich_albums = res
        .into_iter()
        .map(|album| (album.album_name, ImmichAlbumId(album.id)))
        .collect::<Vec<_>>();
    // Maps various version of the (immich) album title to immich album id. The title "as-is" takes
    // precedence. We then lookup gphoto album title (variants) in that map.
    let mut m: HashMap<String, Vec<ImmichAlbumId>> = HashMap::new();

    // Remove spaces - some albums have a trailing space.
    for (name, id) in &immich_albums {
        let name = name
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        m.entry(name).or_default().push(id.clone());
        // m.insert(name, id.clone());
    }

    // Unicode normalization. I had "Trip in Graubu\u{308}nden" and "Trip in Graubünden" in albums
    for (name, id) in &immich_albums {
        let name: String = name.nfc().collect();
        m.entry(name).or_default().push(id.clone());
    }
    // This mapping takes precedence in case there are albums with trailing space and without.
    for (name, id) in immich_albums {
        m.entry(name).or_default().push(id.clone());
    }
    Ok(m)
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

    let _ = dotenvy::from_filename(&args.immich_auth)
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
    check_and_update_schema(&pool).await?;

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

    let scan_result = scan(&pool, &args, &multi, &gphoto_client).await?;
    let search_result = search(&multi, &scan_result, &pool, &immich_client).await?;
    write(
        &multi,
        &search_result,
        &scan_result,
        &pool,
        &immich_client,
        &gphoto_client,
    )
    .await?;

    info!(
        "scan result: media_items: {}, albums: {}",
        scan_result.media_items.len(),
        scan_result.albums.len()
    );
    search_result.log_summary();

    println!("stats: {:?}", STATS.lock().unwrap());
    Ok(())
}
