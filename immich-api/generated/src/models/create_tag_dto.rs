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
pub struct CreateTagDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: models::TagTypeEnum,
}

impl CreateTagDto {
    pub fn new(name: String, r#type: models::TagTypeEnum) -> CreateTagDto {
        CreateTagDto {
            name,
            r#type,
        }
    }
}

