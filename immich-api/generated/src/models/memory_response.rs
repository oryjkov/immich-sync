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
pub struct MemoryResponse {
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl MemoryResponse {
    pub fn new(enabled: bool) -> MemoryResponse {
        MemoryResponse {
            enabled,
        }
    }
}

