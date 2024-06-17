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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedLinkCreateDto {
    #[serde(rename = "albumId", skip_serializing_if = "Option::is_none")]
    pub album_id: Option<uuid::Uuid>,
    #[serde(rename = "allowDownload", skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
    #[serde(rename = "allowUpload", skip_serializing_if = "Option::is_none")]
    pub allow_upload: Option<bool>,
    #[serde(rename = "assetIds", skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expiresAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Option<String>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "showMetadata", skip_serializing_if = "Option::is_none")]
    pub show_metadata: Option<bool>,
    #[serde(rename = "type")]
    pub r#type: models::SharedLinkType,
}

impl SharedLinkCreateDto {
    pub fn new(r#type: models::SharedLinkType) -> SharedLinkCreateDto {
        SharedLinkCreateDto {
            album_id: None,
            allow_download: None,
            allow_upload: None,
            asset_ids: None,
            description: None,
            expires_at: None,
            password: None,
            show_metadata: None,
            r#type,
        }
    }
}

