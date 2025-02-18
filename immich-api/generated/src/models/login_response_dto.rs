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
pub struct LoginResponseDto {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    #[serde(rename = "shouldChangePassword")]
    pub should_change_password: bool,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl LoginResponseDto {
    pub fn new(access_token: String, is_admin: bool, name: String, profile_image_path: String, should_change_password: bool, user_email: String, user_id: String) -> LoginResponseDto {
        LoginResponseDto {
            access_token,
            is_admin,
            name,
            profile_image_path,
            should_change_password,
            user_email,
            user_id,
        }
    }
}

