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
pub struct SearchResponseDto {
    #[serde(rename = "albums")]
    pub albums: Box<models::SearchAlbumResponseDto>,
    #[serde(rename = "assets")]
    pub assets: Box<models::SearchAssetResponseDto>,
}

impl SearchResponseDto {
    pub fn new(albums: models::SearchAlbumResponseDto, assets: models::SearchAssetResponseDto) -> SearchResponseDto {
        SearchResponseDto {
            albums: Box::new(albums),
            assets: Box::new(assets),
        }
    }
}

