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
pub struct UpdateAssetDto {
    #[serde(rename = "dateTimeOriginal", skip_serializing_if = "Option::is_none")]
    pub date_time_original: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isArchived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

impl UpdateAssetDto {
    pub fn new() -> UpdateAssetDto {
        UpdateAssetDto {
            date_time_original: None,
            description: None,
            is_archived: None,
            is_favorite: None,
            latitude: None,
            longitude: None,
        }
    }
}

