use anyhow::Context;
use chrono::{DateTime, Utc};
use immich_api::models;
use itertools::Itertools;
use std::mem;

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct VideoMetadata {
    camera_make: Option<String>,
    camera_model: Option<String>,
}
impl From<&gphotos_api::models::MediaItemMediaMetadataVideo> for VideoMetadata {
    fn from(value: &gphotos_api::models::MediaItemMediaMetadataVideo) -> Self {
        VideoMetadata {
            camera_make: value.camera_make.clone(),
            camera_model: value.camera_model.clone(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone)]
struct PhotoMetadata {
    camera_make: Option<String>,
    camera_model: Option<String>,
    focal_length: Option<f64>,
    aperture_f_number: Option<f64>,
    iso_equivalent: Option<i32>,
    exposure_time: Option<f64>,
}
impl TryFrom<&gphotos_api::models::MediaItemMediaMetadataPhoto> for PhotoMetadata {
    type Error = anyhow::Error;
    fn try_from(
        value: &gphotos_api::models::MediaItemMediaMetadataPhoto,
    ) -> Result<Self, Self::Error> {
        Ok(PhotoMetadata {
            camera_make: value.camera_make.clone(),
            camera_model: value.camera_model.clone(),
            focal_length: value.focal_length.clone(),
            aperture_f_number: value.aperture_f_number.clone(),
            iso_equivalent: value.iso_equivalent.clone(),
            exposure_time: value
                .exposure_time
                .clone()
                .map(|x| x.trim_end_matches('s').parse())
                .transpose()
                .with_context(|| {
                    format!(
                        "tried converting {:?} to float",
                        value.exposure_time.clone()
                    )
                })?,
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ImageData {
    // Immich has several times in the metadata, gphoto only one. We try to see if any match.
    all_times: Vec<DateTime<Utc>>,
    width: Option<f64>,
    height: Option<f64>,
    photo: Option<PhotoMetadata>,
    video: Option<VideoMetadata>,
}
impl TryFrom<&gphotos_api::models::MediaItemMediaMetadata> for ImageData {
    type Error = anyhow::Error;
    fn try_from(value: &gphotos_api::models::MediaItemMediaMetadata) -> Result<Self, Self::Error> {
        Ok(ImageData {
            all_times: value
                .creation_time
                .clone()
                .map(|t| DateTime::parse_from_rfc3339(&t).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?
                .into_iter()
                .collect(),
            width: value.width.clone().map(|x| x.parse()).transpose()?,
            height: value.height.clone().map(|x| x.parse()).transpose()?,
            photo: value
                .photo
                .clone()
                .map(|x| x.as_ref().try_into())
                .transpose()?,
            video: value.video.clone().map(|x| x.as_ref().into()),
        })
    }
}

fn cmp_h<X: PartialEq>(a: Option<X>, b: Option<X>) -> bool {
    if a != b && a.is_some() {
        return true;
    }
    false
}
fn cmp_hf(a: Option<f64>, b: Option<f64>) -> bool {
    if a.is_some() && b.is_some() && (a.unwrap() - b.unwrap()).abs() > 1e-2 {
        return true;
    }
    false
}
// Compares metadata. Returns false if we have good confidence that metadata differs.
// OTOH true could just mean that there was no metadata present, or it could be indeed the same.
pub fn compare_metadata(a: &ImageData, b: &ImageData) -> bool {
    let mut a = a.clone();
    let mut b = b.clone();

    let mut has_match = false;
    for t_a in &a.all_times {
        for t_b in &b.all_times {
            if t_a == t_b {
                has_match = true;
                break;
            }
        }
    }
    if !has_match {
        return false;
    }

    if (a.photo.is_some() && b.photo.is_none()) || (b.photo.is_some() && a.photo.is_none()) {
        return false;
    }
    if a.photo.is_some() {
        // gphoto downsizes videos to 1080p, so only look at height and width on photos

        // allow for flips, for some reason immich and gphoto like to flip
        for x in [&mut a, &mut b] {
            if x.width < x.height {
                mem::swap(&mut x.width, &mut x.height);
            }
        }
        if cmp_h(a.width, b.width) {
            // println!("width");
            return false;
        }
        if cmp_h(a.height, b.height) {
            // println!("height");
            return false;
        }
        let a = a.photo.unwrap();
        let b = b.photo.unwrap();

        if cmp_h(a.camera_make, b.camera_make) {
            return false;
        }
        if cmp_h(a.camera_model, b.camera_model) {
            return false;
        }
        if cmp_h(a.iso_equivalent, b.iso_equivalent) {
            return false;
        }
        if cmp_hf(a.focal_length, b.focal_length) {
            return false;
        }
        if cmp_hf(a.aperture_f_number, b.aperture_f_number) {
            return false;
        }
        if cmp_hf(a.exposure_time, b.exposure_time) {
            return false;
        }
    }

    if a.video.is_some() {
        let a = a.video.unwrap();
        let b = b.video.unwrap();

        if cmp_h(a.camera_make, b.camera_make) {
            return false;
        }
        if cmp_h(a.camera_model, b.camera_model) {
            return false;
        }
    }

    true
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
                    p[0].parse::<f64>().unwrap()
                } else if p.len() == 2 {
                    p[0].parse::<f64>().unwrap() / p[1].parse::<f64>().unwrap()
                } else {
                    panic!("strange input for exposure time: {:?}", s);
                }
            });
        let mut all_times = vec![];
        all_times.push(
            DateTime::parse_from_rfc3339(&value.file_created_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap(),
        );
        all_times.push(
            DateTime::parse_from_rfc3339(&value.file_modified_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap(),
        );
        all_times.push(
            DateTime::parse_from_rfc3339(&value.local_date_time)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap(),
        );
        if let Some(exif) = value.exif_info.as_ref() {
            if let Some(dt) = exif.date_time_original.clone().flatten() {
                all_times.push(
                    DateTime::parse_from_rfc3339(&dt)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap(),
                );
            }
            if let Some(dt) = exif.modify_date.clone().flatten() {
                all_times.push(
                    DateTime::parse_from_rfc3339(&dt)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap(),
                );
            }
        }
        let all_times = all_times.into_iter().unique().collect::<Vec<_>>();

        ImageData {
            all_times,
            width: exif
                .as_ref()
                .and_then(|exif| exif.exif_image_width.flatten()),
            height: exif
                .as_ref()
                .and_then(|exif| exif.exif_image_height.flatten()),
            photo: if value.r#type == models::AssetTypeEnum::Image {
                Some(PhotoMetadata {
                    camera_make: exif
                        .as_ref()
                        .and_then(|exif| exif.make.clone().flatten())
                        .and_then(|x| if x == "" { None } else { Some(x) }),
                    camera_model: exif.as_ref().and_then(|exif| {
                        exif.model
                            .clone()
                            .flatten()
                            .and_then(|x| if x == "" { None } else { Some(x) })
                    }),
                    aperture_f_number: exif.as_ref().and_then(|exif| exif.f_number.flatten()),
                    focal_length: exif.as_ref().and_then(|exif| exif.focal_length.flatten()),
                    iso_equivalent: exif
                        .as_ref()
                        .and_then(|exif| exif.iso.flatten().map(|x| x as i32)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use models::AssetResponseDto;

    #[test]
    fn test_from_immich_empty_camera_string() {
        let immich_metadata = r#"{"checksum":"/S+LG4j0jYDdQHJb4YfjrB6apjk=","deviceAssetId":"10784","deviceId":"6baab4b466900b9a65c66d93933952a5b7c9b003a499ac6a9f01e31a14bb19c4","duplicateId":null,"duration":"0:00:00.00000","exifInfo":{"city":null,"country":null,"dateTimeOriginal":"2024-07-08T18:03:51.000Z","description":"","exifImageHeight":3840.0,"exifImageWidth":2160.0,"exposureTime":null,"fNumber":null,"fileSizeInByte":3757423,"focalLength":null,"iso":null,"latitude":null,"lensModel":null,"longitude":null,"make":"","model":"","modifyDate":"2024-07-08T20:03:31.000Z","orientation":null,"projectionType":null,"state":null,"timeZone":null},"fileCreatedAt":"2024-07-08T18:03:51.000Z","fileModifiedAt":"2024-07-08T18:03:31.000Z","hasMetadata":true,"id":"c969cef6-8b30-4a64-8207-f65504a63782","isArchived":false,"isFavorite":false,"isOffline":false,"isTrashed":false,"libraryId":null,"livePhotoVideoId":null,"localDateTime":"2024-07-08T18:03:51.000Z","originalFileName":"1720461810927.jpg","originalMimeType":"image/jpeg","originalPath":"upload/upload/4f13d54e-b06a-48dc-8f7e-1d47fffe1425/c7/dc/c7dc4d5a-bf91-4795-b6eb-0340e0f8f6dd.jpg","ownerId":"4f13d54e-b06a-48dc-8f7e-1d47fffe1425","people":[],"resized":true,"stackCount":null,"thumbhash":"3MYJTAi69nZ3ZXSYund3cFcGVw==","type":"IMAGE","updatedAt":"2024-07-08T18:28:15.670Z"}"#;
        let i: ImageData = serde_json::from_str::<AssetResponseDto>(immich_metadata)
            .unwrap()
            .into();
        assert_eq!(i.photo.as_ref().unwrap().camera_make, None);
        assert_eq!(i.photo.as_ref().unwrap().camera_model, None);
    }

    #[test]
    fn test_lax_time_match() {
        // only one of the times matches.
        let immich_metadata = r#"{"checksum":"/S+LG4j0jYDdQHJb4YfjrB6apjk=","deviceAssetId":"10784","deviceId":"6baab4b466900b9a65c66d93933952a5b7c9b003a499ac6a9f01e31a14bb19c4","duplicateId":null,"duration":"0:00:00.00000","exifInfo":{"city":null,"country":null,"dateTimeOriginal":"2024-07-08T18:03:51.000Z","description":"","exifImageHeight":3840.0,"exifImageWidth":2160.0,"exposureTime":null,"fNumber":null,"fileSizeInByte":3757423,"focalLength":null,"iso":null,"latitude":null,"lensModel":null,"longitude":null,"make":"","model":"","modifyDate":"2024-07-08T20:03:31.000Z","orientation":null,"projectionType":null,"state":null,"timeZone":null},"fileCreatedAt":"2024-07-08T18:03:51.000Z","fileModifiedAt":"2024-07-08T18:03:31.000Z","hasMetadata":true,"id":"c969cef6-8b30-4a64-8207-f65504a63782","isArchived":false,"isFavorite":false,"isOffline":false,"isTrashed":false,"libraryId":null,"livePhotoVideoId":null,"localDateTime":"2024-07-08T18:03:51.000Z","originalFileName":"1720461810927.jpg","originalMimeType":"image/jpeg","originalPath":"upload/upload/4f13d54e-b06a-48dc-8f7e-1d47fffe1425/c7/dc/c7dc4d5a-bf91-4795-b6eb-0340e0f8f6dd.jpg","ownerId":"4f13d54e-b06a-48dc-8f7e-1d47fffe1425","people":[],"resized":true,"stackCount":null,"thumbhash":"3MYJTAi69nZ3ZXSYund3cFcGVw==","type":"IMAGE","updatedAt":"2024-07-08T18:28:15.670Z"}"#;
        let i: ImageData = serde_json::from_str::<AssetResponseDto>(immich_metadata)
            .unwrap()
            .into();

        let gphoto_metadata =
            r#"{"creationTime":"2024-07-08T18:03:31Z","width":"2160","height":"3840","photo":{}}"#;
        let g: ImageData =
            (&serde_json::from_str::<gphotos_api::models::MediaItemMediaMetadata>(gphoto_metadata)
                .unwrap())
                .try_into()
                .unwrap();
        assert!(compare_metadata(&g, &i));
    }

    #[test]
    fn test_same() {
        let gphoto_metadata = r#"{"creationTime":"2024-06-30T17:52:38Z","width":"720","height":"1280","video":{"fps":30.0,"status":"READY"}}"#;
        let immich_metadata = r#"{"checksum":"Ps9WRshl3BpZYvqgEMMwUSlZBFk=","deviceAssetId":"10734","deviceId":"6baab4b466900b9a65c66d93933952a5b7c9b003a499ac6a9f01e31a14bb19c4","duplicateId":null,"duration":"00:00:15.763","exifInfo":{"city":"Zernez","country":"Switzerland","dateTimeOriginal":"2024-06-30T17:52:38.000Z","description":"","exifImageHeight":720.0,"exifImageWidth":1280.0,"exposureTime":null,"fNumber":null,"fileSizeInByte":14396657,"focalLength":null,"iso":null,"latitude":46.7098,"lensModel":null,"longitude":10.0893,"make":null,"model":null,"modifyDate":"2024-06-30T17:52:38.000Z","orientation":"6","projectionType":null,"state":"Grisons","timeZone":"Europe/Zurich"},"fileCreatedAt":"2024-06-30T17:52:38.000Z","fileModifiedAt":"2024-06-30T17:52:38.000Z","hasMetadata":true,"id":"5297db68-2777-45a6-9ad5-82751da9f4a5","isArchived":false,"isFavorite":false,"isOffline":false,"isTrashed":false,"libraryId":null,"livePhotoVideoId":null,"localDateTime":"2024-06-30T19:52:38.000Z","originalFileName":"20240630_195222.mp4","originalMimeType":"video/mp4","originalPath":"upload/upload/4f13d54e-b06a-48dc-8f7e-1d47fffe1425/d3/7c/d37cb5e3-0cca-4491-bc28-6a1192863cbd.mp4","ownerId":"4f13d54e-b06a-48dc-8f7e-1d47fffe1425","people":[],"resized":true,"stackCount":null,"thumbhash":"o1gGHAbS/KjYWK2Qh2e0r1b/Gw==","type":"VIDEO","updatedAt":"2024-06-30T17:57:56.245Z"}"#;
        let g: ImageData =
            (&serde_json::from_str::<gphotos_api::models::MediaItemMediaMetadata>(gphoto_metadata)
                .unwrap())
                .try_into()
                .unwrap();
        let i: ImageData = serde_json::from_str::<AssetResponseDto>(immich_metadata)
            .unwrap()
            .into();

        assert!(compare_metadata(&g, &i));
    }
    #[test]
    fn test_from() {
        let gphoto_metadata = r#"{"creationTime":"2024-07-08T17:16:59.437Z","width":"4624","height":"3468","photo":{"cameraMake":"samsung","cameraModel":"SM-A536B","focalLength":5.23,"apertureFNumber":1.8,"isoEquivalent":500,"exposureTime":"0.030303031s"}}"#;
        let _: ImageData =
            (&serde_json::from_str::<gphotos_api::models::MediaItemMediaMetadata>(gphoto_metadata)
                .unwrap())
                .try_into()
                .unwrap();
    }
    #[test]
    fn test_different() {
        let gphoto_metadata = r#"{"creationTime":"2024-06-29T21:57:43Z","width":"568","height":"320","video":{"fps":30.0,"status":"READY"}}"#;
        let immich_metadata = r#"{"checksum":"J4KEA6/2Z1Azmn2mkzEX83Gt3Dg=","deviceAssetId":"IMG_7065.mov-123783048","deviceId":"dsk","duplicateId":null,"duration":"00:00:44.241","exifInfo":{"city":null,"country":null,"dateTimeOriginal":"2023-05-28T14:54:38.000Z","description":"","exifImageHeight":1080.0,"exifImageWidth":1920.0,"exposureTime":null,"fNumber":null,"fileSizeInByte":123783048,"focalLength":null,"iso":null,"latitude":null,"lensModel":null,"longitude":null,"make":"Apple","model":"iPhone 13 Pro","modifyDate":"2023-05-29T10:45:35.000Z","orientation":"1","projectionType":null,"state":null,"timeZone":"UTC+2"},"fileCreatedAt":"2023-05-28T14:54:38.000Z","fileModifiedAt":"2023-05-28T14:54:38.000Z","hasMetadata":true,"id":"93997389-06a9-4f25-9cac-c981af0dcaa6","isArchived":false,"isFavorite":false,"isOffline":false,"isTrashed":false,"libraryId":null,"livePhotoVideoId":null,"localDateTime":"2023-05-28T16:54:38.000Z","originalFileName":"IMG_7065.mov","originalMimeType":"video/quicktime","originalPath":"upload/upload/4f13d54e-b06a-48dc-8f7e-1d47fffe1425/c6/6c/c66cdbf9-a492-4c7b-840a-d7250a8cd3ca.mov","ownerId":"4f13d54e-b06a-48dc-8f7e-1d47fffe1425","people":[],"resized":true,"stackCount":null,"thumbhash":"LvgNDIRgiXeIeIh3h3gClwJ1aA==","type":"VIDEO","updatedAt":"2024-06-18T14:45:26.512Z"}"#;
        let g: ImageData =
            (&serde_json::from_str::<gphotos_api::models::MediaItemMediaMetadata>(gphoto_metadata)
                .unwrap())
                .try_into()
                .unwrap();
        let i: ImageData = serde_json::from_str::<AssetResponseDto>(immich_metadata)
            .unwrap()
            .into();

        assert!(!compare_metadata(&g, &i));
    }
    #[test]
    fn test_video_ignores_height_width() {
        let gphoto_metadata = r#"{"creationTime":"2024-07-14T14:44:38Z","width":"1080","height":"1920","video":{"cameraMake":"Insta360","cameraModel":"One X2.VIDEO_NORMAL","fps":29.97002997002997,"status":"READY"}}"#;
        let immich_metadata = r#"{"checksum":"VhKISX5gwfC4Ehp/48JgTBcyNNM=","deviceAssetId":"10804","deviceId":"6baab4b466900b9a65c66d93933952a5b7c9b003a499ac6a9f01e31a14bb19c4","duplicateId":null,"duration":"00:02:42.228","exifInfo":{"city":null,"country":null,"dateTimeOriginal":"2024-07-14T14:45:00.000Z","description":"","exifImageHeight":2560.0,"exifImageWidth":1440.0,"exposureTime":null,"fNumber":null,"fileSizeInByte":851900357,"focalLength":null,"iso":null,"latitude":null,"lensModel":null,"longitude":null,"make":"Insta360","model":"One X2.VIDEO_NORMAL","modifyDate":"2024-07-14T14:44:38.000Z","orientation":"1","projectionType":null,"state":null,"timeZone":"UTC"},"fileCreatedAt":"2024-07-14T14:45:00.000Z","fileModifiedAt":"2024-07-14T14:44:38.000Z","hasMetadata":true,"id":"2d06e4f8-6b18-4482-b826-c3042a6da0ad","isArchived":false,"isFavorite":false,"isOffline":false,"isTrashed":false,"libraryId":null,"livePhotoVideoId":null,"localDateTime":"2024-07-14T14:45:00.000Z","originalFileName":"20240714_163840_461.mp4","originalMimeType":"video/mp4","originalPath":"upload/upload/4f13d54e-b06a-48dc-8f7e-1d47fffe1425/d0/cc/d0cc1741-ae63-40c9-baaa-bcb1a7528294.mp4","ownerId":"4f13d54e-b06a-48dc-8f7e-1d47fffe1425","people":[],"resized":true,"stackCount":null,"thumbhash":"oOcVLAKF+HaIiIV3l3iGYDoFsw==","type":"VIDEO","updatedAt":"2024-07-14T14:53:52.066Z"}"#;
        let g: ImageData =
            (&serde_json::from_str::<gphotos_api::models::MediaItemMediaMetadata>(gphoto_metadata)
                .unwrap())
                .try_into()
                .unwrap();
        let i: ImageData = serde_json::from_str::<AssetResponseDto>(immich_metadata)
            .unwrap()
            .into();

        assert!(compare_metadata(&g, &i));
    }
}
