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
pub struct SystemConfigTemplateStorageOptionDto {
    #[serde(rename = "dayOptions")]
    pub day_options: Vec<String>,
    #[serde(rename = "hourOptions")]
    pub hour_options: Vec<String>,
    #[serde(rename = "minuteOptions")]
    pub minute_options: Vec<String>,
    #[serde(rename = "monthOptions")]
    pub month_options: Vec<String>,
    #[serde(rename = "presetOptions")]
    pub preset_options: Vec<String>,
    #[serde(rename = "secondOptions")]
    pub second_options: Vec<String>,
    #[serde(rename = "weekOptions")]
    pub week_options: Vec<String>,
    #[serde(rename = "yearOptions")]
    pub year_options: Vec<String>,
}

impl SystemConfigTemplateStorageOptionDto {
    pub fn new(day_options: Vec<String>, hour_options: Vec<String>, minute_options: Vec<String>, month_options: Vec<String>, preset_options: Vec<String>, second_options: Vec<String>, week_options: Vec<String>, year_options: Vec<String>) -> SystemConfigTemplateStorageOptionDto {
        SystemConfigTemplateStorageOptionDto {
            day_options,
            hour_options,
            minute_options,
            month_options,
            preset_options,
            second_options,
            week_options,
            year_options,
        }
    }
}

