/*
 * Google Photos API
 *
 * API for accessing Google Photos functionalities.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaItemMediaMetadataPhoto {
    /// Camera make of the photo.
    #[serde(rename = "cameraMake", skip_serializing_if = "Option::is_none")]
    pub camera_make: Option<String>,
    /// Camera model of the photo.
    #[serde(rename = "cameraModel", skip_serializing_if = "Option::is_none")]
    pub camera_model: Option<String>,
    /// Focal length of the photo.
    #[serde(rename = "focalLength", skip_serializing_if = "Option::is_none")]
    pub focal_length: Option<f64>,
    /// Aperture f-number of the photo.
    #[serde(rename = "apertureFNumber", skip_serializing_if = "Option::is_none")]
    pub aperture_f_number: Option<f64>,
    /// ISO equivalent of the photo.
    #[serde(rename = "isoEquivalent", skip_serializing_if = "Option::is_none")]
    pub iso_equivalent: Option<i32>,
    /// Exposure time of the photo.
    #[serde(rename = "exposureTime", skip_serializing_if = "Option::is_none")]
    pub exposure_time: Option<String>,
}

impl MediaItemMediaMetadataPhoto {
    pub fn new() -> MediaItemMediaMetadataPhoto {
        MediaItemMediaMetadataPhoto {
            camera_make: None,
            camera_model: None,
            focal_length: None,
            aperture_f_number: None,
            iso_equivalent: None,
            exposure_time: None,
        }
    }
}

