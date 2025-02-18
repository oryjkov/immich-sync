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
pub struct ReverseGeocodingStateResponseDto {
    #[serde(rename = "lastImportFileName", deserialize_with = "Option::deserialize")]
    pub last_import_file_name: Option<String>,
    #[serde(rename = "lastUpdate", deserialize_with = "Option::deserialize")]
    pub last_update: Option<String>,
}

impl ReverseGeocodingStateResponseDto {
    pub fn new(last_import_file_name: Option<String>, last_update: Option<String>) -> ReverseGeocodingStateResponseDto {
        ReverseGeocodingStateResponseDto {
            last_import_file_name,
            last_update,
        }
    }
}

