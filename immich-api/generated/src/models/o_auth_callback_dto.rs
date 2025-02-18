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
pub struct OAuthCallbackDto {
    #[serde(rename = "url")]
    pub url: String,
}

impl OAuthCallbackDto {
    pub fn new(url: String) -> OAuthCallbackDto {
        OAuthCallbackDto {
            url,
        }
    }
}

