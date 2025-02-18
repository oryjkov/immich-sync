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
pub struct ChangePasswordDto {
    #[serde(rename = "newPassword")]
    pub new_password: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl ChangePasswordDto {
    pub fn new(new_password: String, password: String) -> ChangePasswordDto {
        ChangePasswordDto {
            new_password,
            password,
        }
    }
}

