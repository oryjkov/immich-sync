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
pub struct FacialRecognitionConfig {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "maxDistance")]
    pub max_distance: f64,
    #[serde(rename = "minFaces")]
    pub min_faces: i32,
    #[serde(rename = "minScore")]
    pub min_score: f64,
    #[serde(rename = "modelName")]
    pub model_name: String,
}

impl FacialRecognitionConfig {
    pub fn new(enabled: bool, max_distance: f64, min_faces: i32, min_score: f64, model_name: String) -> FacialRecognitionConfig {
        FacialRecognitionConfig {
            enabled,
            max_distance,
            min_faces,
            min_score,
            model_name,
        }
    }
}

