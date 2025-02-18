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
pub struct DownloadResponseDto {
    #[serde(rename = "archives")]
    pub archives: Vec<models::DownloadArchiveInfo>,
    #[serde(rename = "totalSize")]
    pub total_size: i32,
}

impl DownloadResponseDto {
    pub fn new(archives: Vec<models::DownloadArchiveInfo>, total_size: i32) -> DownloadResponseDto {
        DownloadResponseDto {
            archives,
            total_size,
        }
    }
}

