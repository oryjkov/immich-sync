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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetMediaStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "replaced")]
    Replaced,
    #[serde(rename = "duplicate")]
    Duplicate,

}

impl ToString for AssetMediaStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Created => String::from("created"),
            Self::Replaced => String::from("replaced"),
            Self::Duplicate => String::from("duplicate"),
        }
    }
}

impl Default for AssetMediaStatus {
    fn default() -> AssetMediaStatus {
        Self::Created
    }
}

