// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/skipSegments`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/skipSegments
use serde::{Serialize, Deserialize};


/// url params
#[derive(Serialize, Deserialize, Debug)]
pub struct SkipSegmentsRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(rename = "startTime")]
    start_time: f64,
    #[serde(rename = "endTime")]
    end_time: f64,
    category: String,
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
    service: Option<String>,
    #[serde(rename = "videoDuration")]
    video_duration: f64,
    #[serde(rename = "actionType")]
    action_type: String,
    description: String
}

/// payload
#[derive(Serialize, Deserialize, Debug)]
pub struct SkipSegmentsBodyRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
    service: Option<String>,
    #[serde(rename = "videoDuration")]
    video_duration: Option<f64>,
    segments: Vec<Segment>
}

/// payload
#[derive(Serialize, Deserialize, Debug)]
pub struct SkipSegmentsResponse(Vec<SkipSegmentResponse>);

#[derive(Serialize, Deserialize, Debug)]
pub struct SkipSegmentResponse {
    #[serde(rename = "UUID")]
    uuid: String,
    category: String,
    segment: [f64; 2]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Segment {
    segment: [f64; 2],
    category: String,
    #[serde(rename = "actionType")]
    action_type: Option<String>,
    description: String
}