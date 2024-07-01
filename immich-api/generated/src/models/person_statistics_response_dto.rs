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
pub struct PersonStatisticsResponseDto {
    #[serde(rename = "assets")]
    pub assets: i32,
}

impl PersonStatisticsResponseDto {
    pub fn new(assets: i32) -> PersonStatisticsResponseDto {
        PersonStatisticsResponseDto {
            assets,
        }
    }
}

