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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetTypeEnum {
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "AUDIO")]
    Audio,
    #[serde(rename = "OTHER")]
    Other,

}

impl std::fmt::Display for AssetTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Image => write!(f, "IMAGE"),
            Self::Video => write!(f, "VIDEO"),
            Self::Audio => write!(f, "AUDIO"),
            Self::Other => write!(f, "OTHER"),
        }
    }
}

impl Default for AssetTypeEnum {
    fn default() -> AssetTypeEnum {
        Self::Image
    }
}

