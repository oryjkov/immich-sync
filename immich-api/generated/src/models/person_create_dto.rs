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
pub struct PersonCreateDto {
    /// Person date of birth. Note: the mobile app cannot currently set the birth date to null.
    #[serde(rename = "birthDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<Option<String>>,
    /// Person visibility
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    /// Person name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PersonCreateDto {
    pub fn new() -> PersonCreateDto {
        PersonCreateDto {
            birth_date: None,
            is_hidden: None,
            name: None,
        }
    }
}

