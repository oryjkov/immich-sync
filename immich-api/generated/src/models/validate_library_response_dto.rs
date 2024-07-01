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
pub struct ValidateLibraryResponseDto {
    #[serde(rename = "importPaths", skip_serializing_if = "Option::is_none")]
    pub import_paths: Option<Vec<models::ValidateLibraryImportPathResponseDto>>,
}

impl ValidateLibraryResponseDto {
    pub fn new() -> ValidateLibraryResponseDto {
        ValidateLibraryResponseDto {
            import_paths: None,
        }
    }
}

