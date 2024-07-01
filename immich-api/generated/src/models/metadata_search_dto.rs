/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.106.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSearchDto {
    #[serde(rename = "checksum", skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "createdAfter", skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "createdBefore", skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    #[serde(rename = "deviceAssetId", skip_serializing_if = "Option::is_none")]
    pub device_asset_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "encodedVideoPath", skip_serializing_if = "Option::is_none")]
    pub encoded_video_path: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "isArchived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "isEncoded", skip_serializing_if = "Option::is_none")]
    pub is_encoded: Option<bool>,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "isMotion", skip_serializing_if = "Option::is_none")]
    pub is_motion: Option<bool>,
    #[serde(rename = "isNotInAlbum", skip_serializing_if = "Option::is_none")]
    pub is_not_in_album: Option<bool>,
    #[serde(rename = "isOffline", skip_serializing_if = "Option::is_none")]
    pub is_offline: Option<bool>,
    #[serde(rename = "isVisible", skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
    #[serde(rename = "lensModel", skip_serializing_if = "Option::is_none")]
    pub lens_model: Option<String>,
    #[serde(rename = "libraryId", skip_serializing_if = "Option::is_none")]
    pub library_id: Option<uuid::Uuid>,
    #[serde(rename = "make", skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<models::AssetOrder>,
    #[serde(rename = "originalFileName", skip_serializing_if = "Option::is_none")]
    pub original_file_name: Option<String>,
    #[serde(rename = "originalPath", skip_serializing_if = "Option::is_none")]
    pub original_path: Option<String>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<f64>,
    #[serde(rename = "personIds", skip_serializing_if = "Option::is_none")]
    pub person_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "previewPath", skip_serializing_if = "Option::is_none")]
    pub preview_path: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "takenAfter", skip_serializing_if = "Option::is_none")]
    pub taken_after: Option<String>,
    #[serde(rename = "takenBefore", skip_serializing_if = "Option::is_none")]
    pub taken_before: Option<String>,
    #[serde(rename = "thumbnailPath", skip_serializing_if = "Option::is_none")]
    pub thumbnail_path: Option<String>,
    #[serde(rename = "trashedAfter", skip_serializing_if = "Option::is_none")]
    pub trashed_after: Option<String>,
    #[serde(rename = "trashedBefore", skip_serializing_if = "Option::is_none")]
    pub trashed_before: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::AssetTypeEnum>,
    #[serde(rename = "updatedAfter", skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<String>,
    #[serde(rename = "updatedBefore", skip_serializing_if = "Option::is_none")]
    pub updated_before: Option<String>,
    #[serde(rename = "withArchived", skip_serializing_if = "Option::is_none")]
    pub with_archived: Option<bool>,
    #[serde(rename = "withDeleted", skip_serializing_if = "Option::is_none")]
    pub with_deleted: Option<bool>,
    #[serde(rename = "withExif", skip_serializing_if = "Option::is_none")]
    pub with_exif: Option<bool>,
    #[serde(rename = "withPeople", skip_serializing_if = "Option::is_none")]
    pub with_people: Option<bool>,
    #[serde(rename = "withStacked", skip_serializing_if = "Option::is_none")]
    pub with_stacked: Option<bool>,
}

impl MetadataSearchDto {
    pub fn new() -> MetadataSearchDto {
        MetadataSearchDto {
            checksum: None,
            city: None,
            country: None,
            created_after: None,
            created_before: None,
            device_asset_id: None,
            device_id: None,
            encoded_video_path: None,
            id: None,
            is_archived: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_not_in_album: None,
            is_offline: None,
            is_visible: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            order: None,
            original_file_name: None,
            original_path: None,
            page: None,
            person_ids: None,
            preview_path: None,
            size: None,
            state: None,
            taken_after: None,
            taken_before: None,
            thumbnail_path: None,
            trashed_after: None,
            trashed_before: None,
            r#type: None,
            updated_after: None,
            updated_before: None,
            with_archived: None,
            with_deleted: None,
            with_exif: None,
            with_people: None,
            with_stacked: None,
        }
    }
}

