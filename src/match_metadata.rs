use chrono::{DateTime, Utc};
use immich_api::models;
use serde::Deserialize;
use std::mem;

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
    exposure_time: Option<f64>,
}
fn deserialize_exposure_time<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(exp_s) = s {
        Ok(Some(
            exp_s
                .trim_end_matches('s')
                .parse::<f64>()
                .map_err(serde::de::Error::custom)?,
        ))
    } else {
        Ok(None)
    }
}
fn deserialize_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(exp_s) = s {
        Ok(Some(
            exp_s.parse::<f64>().map_err(serde::de::Error::custom)?,
        ))
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
pub struct ImageData {
    #[serde(default, deserialize_with = "deserialize_creation_time")]
    creation_time: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "deserialize_f64")]
    width: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_f64")]
    height: Option<f64>,
    photo: Option<PhotoMetadata>,
    video: Option<VideoMetadata>,
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

    if cmp_h(a.creation_time, b.creation_time) {
        println!("creation: {:?} {:?}", a.creation_time, b.creation_time);
        return false;
    }
    // allow for flips, for some reason immich and gphoto like to flip
    for x in [&mut a, &mut b] {
        if x.width < x.height {
            mem::swap(&mut x.width, &mut x.height);
        }
    }
    if cmp_h(a.width, b.width) {
        println!("width");
        return false;
    }
    if cmp_h(a.height, b.height) {
        println!("height");
        return false;
    }
    if (a.photo.is_some() && b.photo.is_none()) || (b.photo.is_some() && a.photo.is_none()) {
        return false;
    }
    if a.photo.is_some() {
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
        ImageData {
            creation_time: Some(
                DateTime::parse_from_rfc3339(&value.file_created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap(),
            ),
            width: exif
                .as_ref()
                .and_then(|exif| exif.exif_image_width.flatten()),
            height: exif
                .as_ref()
                .and_then(|exif| exif.exif_image_height.flatten()),
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

#[cfg(test)]
mod tests {
    use models::AssetResponseDto;

    use super::*;
    #[test]
    fn test_same() {
        let gphoto_metadata = r#"{"creationTime":"2024-06-30T17:52:38Z","width":"720","height":"1280","video":{"fps":30.0,"status":"READY"}}"#;
        let immich_metadata = r#"{"checksum":"Ps9WRshl3BpZYvqgEMMwUSlZBFk=","deviceAssetId":"10734","deviceId":"6baab4b466900b9a65c66d93933952a5b7c9b003a499ac6a9f01e31a14bb19c4","duplicateId":null,"duration":"00:00:15.763","exifInfo":{"city":"Zernez","country":"Switzerland","dateTimeOriginal":"2024-06-30T17:52:38.000Z","description":"","exifImageHeight":720.0,"exifImageWidth":1280.0,"exposureTime":null,"fNumber":null,"fileSizeInByte":14396657,"focalLength":null,"iso":null,"latitude":46.7098,"lensModel":null,"longitude":10.0893,"make":null,"model":null,"modifyDate":"2024-06-30T17:52:38.000Z","orientation":"6","projectionType":null,"state":"Grisons","timeZone":"Europe/Zurich"},"fileCreatedAt":"2024-06-30T17:52:38.000Z","fileModifiedAt":"2024-06-30T17:52:38.000Z","hasMetadata":true,"id":"5297db68-2777-45a6-9ad5-82751da9f4a5","isArchived":false,"isFavorite":false,"isOffline":false,"isTrashed":false,"libraryId":null,"livePhotoVideoId":null,"localDateTime":"2024-06-30T19:52:38.000Z","originalFileName":"20240630_195222.mp4","originalMimeType":"video/mp4","originalPath":"upload/upload/4f13d54e-b06a-48dc-8f7e-1d47fffe1425/d3/7c/d37cb5e3-0cca-4491-bc28-6a1192863cbd.mp4","ownerId":"4f13d54e-b06a-48dc-8f7e-1d47fffe1425","people":[],"resized":true,"stackCount":null,"thumbhash":"o1gGHAbS/KjYWK2Qh2e0r1b/Gw==","type":"VIDEO","updatedAt":"2024-06-30T17:57:56.245Z"}"#;
        let g: ImageData = serde_json::from_str(gphoto_metadata).unwrap();
        let i: ImageData = serde_json::from_str::<AssetResponseDto>(immich_metadata)
            .unwrap()
            .into();

        assert!(compare_metadata(&g, &i));
    }

    #[test]
    fn test_different() {
        let gphoto_metadata = r#"{"creationTime":"2024-06-29T21:57:43Z","width":"568","height":"320","video":{"fps":30.0,"status":"READY"}}"#;
        let immich_metadata = r#"{"checksum":"J4KEA6/2Z1Azmn2mkzEX83Gt3Dg=","deviceAssetId":"IMG_7065.mov-123783048","deviceId":"dsk","duplicateId":null,"duration":"00:00:44.241","exifInfo":{"city":null,"country":null,"dateTimeOriginal":"2023-05-28T14:54:38.000Z","description":"","exifImageHeight":1080.0,"exifImageWidth":1920.0,"exposureTime":null,"fNumber":null,"fileSizeInByte":123783048,"focalLength":null,"iso":null,"latitude":null,"lensModel":null,"longitude":null,"make":"Apple","model":"iPhone 13 Pro","modifyDate":"2023-05-29T10:45:35.000Z","orientation":"1","projectionType":null,"state":null,"timeZone":"UTC+2"},"fileCreatedAt":"2023-05-28T14:54:38.000Z","fileModifiedAt":"2023-05-28T14:54:38.000Z","hasMetadata":true,"id":"93997389-06a9-4f25-9cac-c981af0dcaa6","isArchived":false,"isFavorite":false,"isOffline":false,"isTrashed":false,"libraryId":null,"livePhotoVideoId":null,"localDateTime":"2023-05-28T16:54:38.000Z","originalFileName":"IMG_7065.mov","originalMimeType":"video/quicktime","originalPath":"upload/upload/4f13d54e-b06a-48dc-8f7e-1d47fffe1425/c6/6c/c66cdbf9-a492-4c7b-840a-d7250a8cd3ca.mov","ownerId":"4f13d54e-b06a-48dc-8f7e-1d47fffe1425","people":[],"resized":true,"stackCount":null,"thumbhash":"LvgNDIRgiXeIeIh3h3gClwJ1aA==","type":"VIDEO","updatedAt":"2024-06-18T14:45:26.512Z"}"#;
        let g: ImageData = serde_json::from_str(gphoto_metadata).unwrap();
        let i: ImageData = serde_json::from_str::<AssetResponseDto>(immich_metadata)
            .unwrap()
            .into();

        assert!(!compare_metadata(&g, &i));
    }
}
