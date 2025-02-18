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
pub enum PathEntityType {
    #[serde(rename = "asset")]
    Asset,
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "user")]
    User,

}

impl std::fmt::Display for PathEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Asset => write!(f, "asset"),
            Self::Person => write!(f, "person"),
            Self::User => write!(f, "user"),
        }
    }
}

impl Default for PathEntityType {
    fn default() -> PathEntityType {
        Self::Asset
    }
}

