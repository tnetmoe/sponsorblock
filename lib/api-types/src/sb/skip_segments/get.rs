// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/skipSegments`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/skipSegments
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct SkipSegmentsRequestQuery {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(flatten)]
    pub category: Category,
    #[serde(flatten)]
    pub required_segment: RequiredSegment,
    #[serde(flatten)]
    pub action_type: ActionType,
    pub service: Option<String>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct SkipSegmentsResponseBody(Vec<SkipSegmentResponse>);

#[derive(Serialize, Deserialize)]
pub struct SkipSegmentResponse {
    pub segment: [f64; 2],
    #[serde(rename = "UUID")]
    pub uuid: String,
    pub category: String,
    #[serde(rename = "videoDuration")]
    pub video_duration: f64,
    #[serde(rename = "actionType")]
    pub action_type: String,
    pub locked: i64,
    pub votes: i64,
    pub description: String
}

#[derive(Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "category")]
    Category(String),
    #[serde(rename = "categories")]
    Categories(Vec<String>)
}

#[derive(Serialize, Deserialize)]
pub enum RequiredSegment {
    #[serde(rename = "requiredSegment")]
    RequiredSegment(String),
    #[serde(rename = "requiredSegments")]
    RequiredSegments(Vec<String>)
}

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "actionType")]
    ActionType(String),
    #[serde(rename = "actionTypes")]
    ActionTypes(Vec<String>)
}