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
pub struct DuplicateResponseDto {
    #[serde(rename = "assets")]
    pub assets: Vec<models::AssetResponseDto>,
    #[serde(rename = "duplicateId")]
    pub duplicate_id: String,
}

impl DuplicateResponseDto {
    pub fn new(assets: Vec<models::AssetResponseDto>, duplicate_id: String) -> DuplicateResponseDto {
        DuplicateResponseDto {
            assets,
            duplicate_id,
        }
    }
}

