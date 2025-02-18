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
pub struct AlbumSharedAlbumOptions {
    /// Indicates if the album is collaborative.
    #[serde(rename = "isCollaborative", skip_serializing_if = "Option::is_none")]
    pub is_collaborative: Option<bool>,
    /// Indicates if comments are enabled for the album.
    #[serde(rename = "isCommentable", skip_serializing_if = "Option::is_none")]
    pub is_commentable: Option<bool>,
}

impl AlbumSharedAlbumOptions {
    pub fn new() -> AlbumSharedAlbumOptions {
        AlbumSharedAlbumOptions {
            is_collaborative: None,
            is_commentable: None,
        }
    }
}

