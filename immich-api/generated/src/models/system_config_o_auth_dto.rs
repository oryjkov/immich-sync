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
pub struct SystemConfigOAuthDto {
    #[serde(rename = "autoLaunch")]
    pub auto_launch: bool,
    #[serde(rename = "autoRegister")]
    pub auto_register: bool,
    #[serde(rename = "buttonText")]
    pub button_text: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[serde(rename = "defaultStorageQuota")]
    pub default_storage_quota: f64,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "issuerUrl")]
    pub issuer_url: String,
    #[serde(rename = "mobileOverrideEnabled")]
    pub mobile_override_enabled: bool,
    #[serde(rename = "mobileRedirectUri")]
    pub mobile_redirect_uri: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: String,
    #[serde(rename = "storageLabelClaim")]
    pub storage_label_claim: String,
    #[serde(rename = "storageQuotaClaim")]
    pub storage_quota_claim: String,
}

impl SystemConfigOAuthDto {
    pub fn new(auto_launch: bool, auto_register: bool, button_text: String, client_id: String, client_secret: String, default_storage_quota: f64, enabled: bool, issuer_url: String, mobile_override_enabled: bool, mobile_redirect_uri: String, scope: String, signing_algorithm: String, storage_label_claim: String, storage_quota_claim: String) -> SystemConfigOAuthDto {
        SystemConfigOAuthDto {
            auto_launch,
            auto_register,
            button_text,
            client_id,
            client_secret,
            default_storage_quota,
            enabled,
            issuer_url,
            mobile_override_enabled,
            mobile_redirect_uri,
            scope,
            signing_algorithm,
            storage_label_claim,
            storage_quota_claim,
        }
    }
}

