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
pub struct AlbumUserCreateDto {
    #[serde(rename = "role")]
    pub role: models::AlbumUserRole,
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
}

impl AlbumUserCreateDto {
    pub fn new(role: models::AlbumUserRole, user_id: uuid::Uuid) -> AlbumUserCreateDto {
        AlbumUserCreateDto {
            role,
            user_id,
        }
    }
}

