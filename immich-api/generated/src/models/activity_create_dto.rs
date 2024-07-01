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
pub struct ActivityCreateDto {
    #[serde(rename = "albumId")]
    pub album_id: uuid::Uuid,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<uuid::Uuid>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "type")]
    pub r#type: models::ReactionType,
}

impl ActivityCreateDto {
    pub fn new(album_id: uuid::Uuid, r#type: models::ReactionType) -> ActivityCreateDto {
        ActivityCreateDto {
            album_id,
            asset_id: None,
            comment: None,
            r#type,
        }
    }
}

