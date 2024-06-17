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
pub struct SearchExploreItem {
    #[serde(rename = "data")]
    pub data: Box<models::AssetResponseDto>,
    #[serde(rename = "value")]
    pub value: String,
}

impl SearchExploreItem {
    pub fn new(data: models::AssetResponseDto, value: String) -> SearchExploreItem {
        SearchExploreItem {
            data: Box::new(data),
            value,
        }
    }
}

