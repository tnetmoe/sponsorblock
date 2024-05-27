// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/segmentInfo`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/segmentInfo
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct SegmentInfoRequest {
    #[serde(flatten)]
    uuid: UUID
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct SegmentInfoResponse(Vec<SegmentInfo>);

#[derive(Serialize, Deserialize)]
pub enum UUID {
    #[serde(rename = "UUID")]
    UUID(String),
    #[serde(rename = "UUIDs")]
    UUIDs(Vec<String>)
}

#[derive(Serialize, Deserialize)]
pub struct SegmentInfo {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(rename = "startTime")]
    start_time: f64,
    #[serde(rename = "endTime")]
    end_time: f64,
    votes: i64,
    locked: i64,
    #[serde(rename = "UUID")]
    uuid: String,
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "timeSubmitted")]
    time_submitted: i64,
    views: i64,
    category: String,
    service: String,
    #[serde(rename = "actionType")]
    action_type: String,
    #[serde(rename = "videoDuration")]
    video_duration: i64,
    hidden: i64,
    reputation: i64,
    #[serde(rename = "shadowHidden")]
    shadow_hidden: i64,
    #[serde(rename = "hashedVideoID")]
    hashed_video_id: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
    description: String
}