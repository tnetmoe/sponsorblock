// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/searchSegments`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/searchSegments
use serde::{Serialize, Deserialize};
use super::skip_segments::get::{Category, ActionType};

/// url params OR json body
#[derive(Serialize, Deserialize)]
pub struct SearchSegmentsRequest {
    #[serde(rename = "videoID")]
    pub video_id: String,
    pub category: Category,
    #[serde(rename = "actionType")]
    pub action_type: ActionType,
    pub service: String,
    pub page: i64,
    #[serde(rename = "minVotes")]
    pub min_votes: i64,
    #[serde(rename = "maxVotes")]
    pub max_votes: i64,
    #[serde(rename = "minViews")]
    pub min_views: i64,
    #[serde(rename = "maxViews")]
    pub max_views: i64,
    pub locked: bool,
    pub hidden: bool,
    pub ignored: bool
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct SearchSegmentsResponse {
    #[serde(rename = "segmentCount")]
    pub segment_count: i64,
    pub page: i64,
    pub segments: Vec<Segment>
}

#[derive(Serialize, Deserialize)]
pub struct Segment {
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "timeSubmitted")]
    pub time_submitted: i64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    pub category: String,
    #[serde(rename = "actionType")]
    pub action_type: String,
    pub votes: i64,
    pub views: i64,
    pub locked: i64,
    pub hidden: i64,
    #[serde(rename = "shadowHidden")]
    pub shadow_hidden: i64,
    #[serde(rename = "userID")]
    pub user_id: String,
    pub description: String
}