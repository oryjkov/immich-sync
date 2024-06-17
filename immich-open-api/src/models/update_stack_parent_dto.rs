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
pub struct UpdateStackParentDto {
    #[serde(rename = "newParentId")]
    pub new_parent_id: uuid::Uuid,
    #[serde(rename = "oldParentId")]
    pub old_parent_id: uuid::Uuid,
}

impl UpdateStackParentDto {
    pub fn new(new_parent_id: uuid::Uuid, old_parent_id: uuid::Uuid) -> UpdateStackParentDto {
        UpdateStackParentDto {
            new_parent_id,
            old_parent_id,
        }
    }
}

