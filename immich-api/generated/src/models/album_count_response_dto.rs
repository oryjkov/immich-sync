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
pub struct AlbumCountResponseDto {
    #[serde(rename = "notShared")]
    pub not_shared: i32,
    #[serde(rename = "owned")]
    pub owned: i32,
    #[serde(rename = "shared")]
    pub shared: i32,
}

impl AlbumCountResponseDto {
    pub fn new(not_shared: i32, owned: i32, shared: i32) -> AlbumCountResponseDto {
        AlbumCountResponseDto {
            not_shared,
            owned,
            shared,
        }
    }
}

