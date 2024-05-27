// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/skipSegments`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/skipSegments
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct SkipSegmentsRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(flatten)]
    category: Category,
    #[serde(flatten)]
    required_segment: RequiredSegment,
    #[serde(flatten)]
    action_type: ActionType,
    service: Option<String>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct SkipSegmentsResponse(Vec<SkipSegmentResponse>);

#[derive(Serialize, Deserialize)]
pub struct SkipSegmentResponse {
    segment: [f64; 2],
    #[serde(rename = "UUID")]
    uuid: String,
    category: String,
    #[serde(rename = "videoDuration")]
    video_duration: f64,
    #[serde(rename = "actionType")]
    action_type: String,
    locked: i64,
    votes: i64,
    description: String
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