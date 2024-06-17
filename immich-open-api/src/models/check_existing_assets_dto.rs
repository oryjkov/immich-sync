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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckExistingAssetsDto {
    #[serde(rename = "deviceAssetIds")]
    pub device_asset_ids: Vec<String>,
    #[serde(rename = "deviceId")]
    pub device_id: String,
}

impl CheckExistingAssetsDto {
    pub fn new(device_asset_ids: Vec<String>, device_id: String) -> CheckExistingAssetsDto {
        CheckExistingAssetsDto {
            device_asset_ids,
            device_id,
        }
    }
}

