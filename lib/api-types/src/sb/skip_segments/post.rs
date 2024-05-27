// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/skipSegments`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/skipSegments
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize, Debug)]
pub struct SkipSegmentsRequestQuery {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(rename = "startTime")]
    pub start_time: f64,
    #[serde(rename = "endTime")]
    pub end_time: f64,
    pub category: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub service: Option<String>,
    #[serde(rename = "videoDuration")]
    pub video_duration: f64,
    #[serde(rename = "actionType")]
    pub action_type: String,
    pub description: String
}

/// payload
#[derive(Serialize, Deserialize, Debug)]
pub struct SkipSegmentsRequestBody {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub service: Option<String>,
    #[serde(rename = "videoDuration")]
    pub video_duration: Option<f64>,
    pub segments: Vec<Segment>
}

/// payload
#[derive(Serialize, Deserialize, Debug)]
pub struct SkipSegmentsResponseBody(Vec<SkipSegmentResponse>);

#[derive(Serialize, Deserialize, Debug)]
pub struct SkipSegmentResponse {
    #[serde(rename = "UUID")]
    pub uuid: String,
    pub category: String,
    pub segment: [f64; 2]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Segment {
    pub segment: [f64; 2],
    pub category: String,
    #[serde(rename = "actionType")]
    pub action_type: Option<String>,
    pub description: String
}