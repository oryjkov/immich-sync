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
pub struct SystemConfigUserDto {
    #[serde(rename = "deleteDelay")]
    pub delete_delay: i32,
}

impl SystemConfigUserDto {
    pub fn new(delete_delay: i32) -> SystemConfigUserDto {
        SystemConfigUserDto {
            delete_delay,
        }
    }
}

