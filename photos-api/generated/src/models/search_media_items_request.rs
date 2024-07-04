/*
 * Google Photos API
 *
 * API for accessing Google Photos functionalities.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchMediaItemsRequest {
    /// ID of the album to search media items in.
    #[serde(rename = "albumId")]
    pub album_id: String,
    /// Maximum number of media items to return.
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// Token to retrieve the next page of results.
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl SearchMediaItemsRequest {
    pub fn new(album_id: String) -> SearchMediaItemsRequest {
        SearchMediaItemsRequest {
            album_id,
            page_size: None,
            page_token: None,
        }
    }
}

