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
pub struct SystemConfigFFmpegDto {
    #[serde(rename = "accel")]
    pub accel: models::TranscodeHwAccel,
    #[serde(rename = "accelDecode")]
    pub accel_decode: bool,
    #[serde(rename = "acceptedAudioCodecs")]
    pub accepted_audio_codecs: Vec<models::AudioCodec>,
    #[serde(rename = "acceptedVideoCodecs")]
    pub accepted_video_codecs: Vec<models::VideoCodec>,
    #[serde(rename = "bframes")]
    pub bframes: i32,
    #[serde(rename = "cqMode")]
    pub cq_mode: models::CqMode,
    #[serde(rename = "crf")]
    pub crf: i32,
    #[serde(rename = "gopSize")]
    pub gop_size: i32,
    #[serde(rename = "maxBitrate")]
    pub max_bitrate: String,
    #[serde(rename = "npl")]
    pub npl: i32,
    #[serde(rename = "preferredHwDevice")]
    pub preferred_hw_device: String,
    #[serde(rename = "preset")]
    pub preset: String,
    #[serde(rename = "refs")]
    pub refs: i32,
    #[serde(rename = "targetAudioCodec")]
    pub target_audio_codec: models::AudioCodec,
    #[serde(rename = "targetResolution")]
    pub target_resolution: String,
    #[serde(rename = "targetVideoCodec")]
    pub target_video_codec: models::VideoCodec,
    #[serde(rename = "temporalAQ")]
    pub temporal_aq: bool,
    #[serde(rename = "threads")]
    pub threads: i32,
    #[serde(rename = "tonemap")]
    pub tonemap: models::ToneMapping,
    #[serde(rename = "transcode")]
    pub transcode: models::TranscodePolicy,
    #[serde(rename = "twoPass")]
    pub two_pass: bool,
}

impl SystemConfigFFmpegDto {
    pub fn new(accel: models::TranscodeHwAccel, accel_decode: bool, accepted_audio_codecs: Vec<models::AudioCodec>, accepted_video_codecs: Vec<models::VideoCodec>, bframes: i32, cq_mode: models::CqMode, crf: i32, gop_size: i32, max_bitrate: String, npl: i32, preferred_hw_device: String, preset: String, refs: i32, target_audio_codec: models::AudioCodec, target_resolution: String, target_video_codec: models::VideoCodec, temporal_aq: bool, threads: i32, tonemap: models::ToneMapping, transcode: models::TranscodePolicy, two_pass: bool) -> SystemConfigFFmpegDto {
        SystemConfigFFmpegDto {
            accel,
            accel_decode,
            accepted_audio_codecs,
            accepted_video_codecs,
            bframes,
            cq_mode,
            crf,
            gop_size,
            max_bitrate,
            npl,
            preferred_hw_device,
            preset,
            refs,
            target_audio_codec,
            target_resolution,
            target_video_codec,
            temporal_aq,
            threads,
            tonemap,
            transcode,
            two_pass,
        }
    }
}

