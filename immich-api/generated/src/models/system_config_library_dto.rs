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
pub struct SystemConfigLibraryDto {
    #[serde(rename = "scan")]
    pub scan: Box<models::SystemConfigLibraryScanDto>,
    #[serde(rename = "watch")]
    pub watch: Box<models::SystemConfigLibraryWatchDto>,
}

impl SystemConfigLibraryDto {
    pub fn new(scan: models::SystemConfigLibraryScanDto, watch: models::SystemConfigLibraryWatchDto) -> SystemConfigLibraryDto {
        SystemConfigLibraryDto {
            scan: Box::new(scan),
            watch: Box::new(watch),
        }
    }
}

