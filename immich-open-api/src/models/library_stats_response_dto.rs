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
pub struct LibraryStatsResponseDto {
    #[serde(rename = "photos")]
    pub photos: i32,
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "usage")]
    pub usage: i64,
    #[serde(rename = "videos")]
    pub videos: i32,
}

impl LibraryStatsResponseDto {
    pub fn new(photos: i32, total: i32, usage: i64, videos: i32) -> LibraryStatsResponseDto {
        LibraryStatsResponseDto {
            photos,
            total,
            usage,
            videos,
        }
    }
}

