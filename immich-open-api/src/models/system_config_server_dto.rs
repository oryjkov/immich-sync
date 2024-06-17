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
pub struct SystemConfigServerDto {
    #[serde(rename = "externalDomain")]
    pub external_domain: String,
    #[serde(rename = "loginPageMessage")]
    pub login_page_message: String,
}

impl SystemConfigServerDto {
    pub fn new(external_domain: String, login_page_message: String) -> SystemConfigServerDto {
        SystemConfigServerDto {
            external_domain,
            login_page_message,
        }
    }
}

