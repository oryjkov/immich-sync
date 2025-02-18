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
pub struct JobStatusDto {
    #[serde(rename = "jobCounts")]
    pub job_counts: Box<models::JobCountsDto>,
    #[serde(rename = "queueStatus")]
    pub queue_status: Box<models::QueueStatusDto>,
}

impl JobStatusDto {
    pub fn new(job_counts: models::JobCountsDto, queue_status: models::QueueStatusDto) -> JobStatusDto {
        JobStatusDto {
            job_counts: Box::new(job_counts),
            queue_status: Box::new(queue_status),
        }
    }
}

