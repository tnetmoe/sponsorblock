// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/segmentInfo`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/segmentInfo
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct SegmentInfoRequestQuery {
    #[serde(flatten)]
    pub uuid: UUID
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct SegmentInfoResponseBody(Vec<SegmentInfo>);

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
    pub video_id: String,
    #[serde(rename = "startTime")]
    pub start_time: f64,
    #[serde(rename = "endTime")]
    pub end_time: f64,
    pub votes: i64,
    pub locked: i64,
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "timeSubmitted")]
    pub time_submitted: i64,
    pub views: i64,
    pub category: String,
    pub service: String,
    #[serde(rename = "actionType")]
    pub action_type: String,
    #[serde(rename = "videoDuration")]
    pub video_duration: i64,
    pub hidden: i64,
    pub reputation: i64,
    #[serde(rename = "shadowHidden")]
    pub shadow_hidden: i64,
    #[serde(rename = "hashedVideoID")]
    pub hashed_video_id: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub description: String
}