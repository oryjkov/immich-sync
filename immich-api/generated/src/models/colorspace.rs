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
pub enum Colorspace {
    #[serde(rename = "srgb")]
    Srgb,
    #[serde(rename = "p3")]
    P3,

}

impl std::fmt::Display for Colorspace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Srgb => write!(f, "srgb"),
            Self::P3 => write!(f, "p3"),
        }
    }
}

impl Default for Colorspace {
    fn default() -> Colorspace {
        Self::Srgb
    }
}

