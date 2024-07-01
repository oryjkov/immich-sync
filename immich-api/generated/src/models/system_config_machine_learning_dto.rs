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
pub struct SystemConfigMachineLearningDto {
    #[serde(rename = "clip")]
    pub clip: Box<models::ClipConfig>,
    #[serde(rename = "duplicateDetection")]
    pub duplicate_detection: Box<models::DuplicateDetectionConfig>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "facialRecognition")]
    pub facial_recognition: Box<models::FacialRecognitionConfig>,
    #[serde(rename = "url")]
    pub url: String,
}

impl SystemConfigMachineLearningDto {
    pub fn new(clip: models::ClipConfig, duplicate_detection: models::DuplicateDetectionConfig, enabled: bool, facial_recognition: models::FacialRecognitionConfig, url: String) -> SystemConfigMachineLearningDto {
        SystemConfigMachineLearningDto {
            clip: Box::new(clip),
            duplicate_detection: Box::new(duplicate_detection),
            enabled,
            facial_recognition: Box::new(facial_recognition),
            url,
        }
    }
}

