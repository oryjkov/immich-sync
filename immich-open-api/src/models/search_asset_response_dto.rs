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
pub struct SearchAssetResponseDto {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "facets")]
    pub facets: Vec<models::SearchFacetResponseDto>,
    #[serde(rename = "items")]
    pub items: Vec<models::AssetResponseDto>,
    #[serde(rename = "nextPage", deserialize_with = "Option::deserialize")]
    pub next_page: Option<String>,
    #[serde(rename = "total")]
    pub total: i32,
}

impl SearchAssetResponseDto {
    pub fn new(count: i32, facets: Vec<models::SearchFacetResponseDto>, items: Vec<models::AssetResponseDto>, next_page: Option<String>, total: i32) -> SearchAssetResponseDto {
        SearchAssetResponseDto {
            count,
            facets,
            items,
            next_page,
            total,
        }
    }
}

