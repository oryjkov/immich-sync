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
pub enum VideoCodec {
    #[serde(rename = "h264")]
    H264,
    #[serde(rename = "hevc")]
    Hevc,
    #[serde(rename = "vp9")]
    Vp9,
    #[serde(rename = "av1")]
    Av1,

}

impl std::fmt::Display for VideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::H264 => write!(f, "h264"),
            Self::Hevc => write!(f, "hevc"),
            Self::Vp9 => write!(f, "vp9"),
            Self::Av1 => write!(f, "av1"),
        }
    }
}

impl Default for VideoCodec {
    fn default() -> VideoCodec {
        Self::H264
    }
}

