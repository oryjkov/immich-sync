/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.106.4
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`get_time_bucket`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTimeBucketError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_time_buckets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTimeBucketsError {
    UnknownValue(serde_json::Value),
}


pub async fn get_time_bucket(configuration: &configuration::Configuration, size: models::TimeBucketSize, time_bucket: &str, album_id: Option<&str>, is_archived: Option<bool>, is_favorite: Option<bool>, is_trashed: Option<bool>, key: Option<&str>, order: Option<models::AssetOrder>, person_id: Option<&str>, user_id: Option<&str>, with_partners: Option<bool>, with_stacked: Option<bool>) -> Result<Vec<models::AssetResponseDto>, Error<GetTimeBucketError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/timeline/bucket", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = album_id {
        local_var_req_builder = local_var_req_builder.query(&[("albumId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_archived {
        local_var_req_builder = local_var_req_builder.query(&[("isArchived", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_favorite {
        local_var_req_builder = local_var_req_builder.query(&[("isFavorite", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_trashed {
        local_var_req_builder = local_var_req_builder.query(&[("isTrashed", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = person_id {
        local_var_req_builder = local_var_req_builder.query(&[("personId", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("size", &size.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("timeBucket", &time_bucket.to_string())]);
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_partners {
        local_var_req_builder = local_var_req_builder.query(&[("withPartners", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_stacked {
        local_var_req_builder = local_var_req_builder.query(&[("withStacked", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTimeBucketError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_time_buckets(configuration: &configuration::Configuration, size: models::TimeBucketSize, album_id: Option<&str>, is_archived: Option<bool>, is_favorite: Option<bool>, is_trashed: Option<bool>, key: Option<&str>, order: Option<models::AssetOrder>, person_id: Option<&str>, user_id: Option<&str>, with_partners: Option<bool>, with_stacked: Option<bool>) -> Result<Vec<models::TimeBucketResponseDto>, Error<GetTimeBucketsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/timeline/buckets", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = album_id {
        local_var_req_builder = local_var_req_builder.query(&[("albumId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_archived {
        local_var_req_builder = local_var_req_builder.query(&[("isArchived", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_favorite {
        local_var_req_builder = local_var_req_builder.query(&[("isFavorite", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_trashed {
        local_var_req_builder = local_var_req_builder.query(&[("isTrashed", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = person_id {
        local_var_req_builder = local_var_req_builder.query(&[("personId", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("size", &size.to_string())]);
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_partners {
        local_var_req_builder = local_var_req_builder.query(&[("withPartners", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_stacked {
        local_var_req_builder = local_var_req_builder.query(&[("withStacked", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTimeBucketsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

