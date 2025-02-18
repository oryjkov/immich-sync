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
pub struct AuditDeletesResponseDto {
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    #[serde(rename = "needsFullSync")]
    pub needs_full_sync: bool,
}

impl AuditDeletesResponseDto {
    pub fn new(ids: Vec<String>, needs_full_sync: bool) -> AuditDeletesResponseDto {
        AuditDeletesResponseDto {
            ids,
            needs_full_sync,
        }
    }
}

