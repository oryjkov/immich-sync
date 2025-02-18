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
pub struct ValidateAccessTokenResponseDto {
    #[serde(rename = "authStatus")]
    pub auth_status: bool,
}

impl ValidateAccessTokenResponseDto {
    pub fn new(auth_status: bool) -> ValidateAccessTokenResponseDto {
        ValidateAccessTokenResponseDto {
            auth_status,
        }
    }
}

