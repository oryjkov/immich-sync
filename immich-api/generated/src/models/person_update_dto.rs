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
pub struct PersonUpdateDto {
    /// Person date of birth. Note: the mobile app cannot currently set the birth date to null.
    #[serde(rename = "birthDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<Option<String>>,
    /// Asset is used to get the feature face thumbnail.
    #[serde(rename = "featureFaceAssetId", skip_serializing_if = "Option::is_none")]
    pub feature_face_asset_id: Option<String>,
    /// Person visibility
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    /// Person name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PersonUpdateDto {
    pub fn new() -> PersonUpdateDto {
        PersonUpdateDto {
            birth_date: None,
            feature_face_asset_id: None,
            is_hidden: None,
            name: None,
        }
    }
}

