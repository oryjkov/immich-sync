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
pub struct ScanLibraryDto {
    #[serde(rename = "refreshAllFiles", skip_serializing_if = "Option::is_none")]
    pub refresh_all_files: Option<bool>,
    #[serde(rename = "refreshModifiedFiles", skip_serializing_if = "Option::is_none")]
    pub refresh_modified_files: Option<bool>,
}

impl ScanLibraryDto {
    pub fn new() -> ScanLibraryDto {
        ScanLibraryDto {
            refresh_all_files: None,
            refresh_modified_files: None,
        }
    }
}

