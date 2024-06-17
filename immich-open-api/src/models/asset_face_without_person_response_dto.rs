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
pub struct AssetFaceWithoutPersonResponseDto {
    #[serde(rename = "boundingBoxX1")]
    pub bounding_box_x1: i32,
    #[serde(rename = "boundingBoxX2")]
    pub bounding_box_x2: i32,
    #[serde(rename = "boundingBoxY1")]
    pub bounding_box_y1: i32,
    #[serde(rename = "boundingBoxY2")]
    pub bounding_box_y2: i32,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "imageHeight")]
    pub image_height: i32,
    #[serde(rename = "imageWidth")]
    pub image_width: i32,
}

impl AssetFaceWithoutPersonResponseDto {
    pub fn new(bounding_box_x1: i32, bounding_box_x2: i32, bounding_box_y1: i32, bounding_box_y2: i32, id: uuid::Uuid, image_height: i32, image_width: i32) -> AssetFaceWithoutPersonResponseDto {
        AssetFaceWithoutPersonResponseDto {
            bounding_box_x1,
            bounding_box_x2,
            bounding_box_y1,
            bounding_box_y2,
            id,
            image_height,
            image_width,
        }
    }
}

