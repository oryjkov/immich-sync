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
pub struct FileReportItemDto {
    #[serde(rename = "checksum", skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "entityId")]
    pub entity_id: uuid::Uuid,
    #[serde(rename = "entityType")]
    pub entity_type: models::PathEntityType,
    #[serde(rename = "pathType")]
    pub path_type: models::PathType,
    #[serde(rename = "pathValue")]
    pub path_value: String,
}

impl FileReportItemDto {
    pub fn new(entity_id: uuid::Uuid, entity_type: models::PathEntityType, path_type: models::PathType, path_value: String) -> FileReportItemDto {
        FileReportItemDto {
            checksum: None,
            entity_id,
            entity_type,
            path_type,
            path_value,
        }
    }
}

