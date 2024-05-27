// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/searchSegments`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/searchSegments
use serde::{Serialize, Deserialize};
use super::skip_segments::get::{Category, ActionType};

/// url params OR json body
#[derive(Serialize, Deserialize)]
pub struct SearchSegmentsRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    category: Category,
    #[serde(rename = "actionType")]
    action_type: ActionType,
    service: String,
    page: i64,
    #[serde(rename = "minVotes")]
    min_votes: i64,
    #[serde(rename = "maxVotes")]
    max_votes: i64,
    #[serde(rename = "minViews")]
    min_views: i64,
    #[serde(rename = "maxViews")]
    max_views: i64,
    locked: bool,
    hidden: bool,
    ignored: bool
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct SearchSegmentsResponse {
    #[serde(rename = "segmentCount")]
    segment_count: i64,
    page: i64,
    segments: Vec<Segment>
}

#[derive(Serialize, Deserialize)]
pub struct Segment {
    #[serde(rename = "UUID")]
    uuid: String,
    #[serde(rename = "timeSubmitted")]
    time_submitted: i64,
    #[serde(rename = "startTime")]
    start_time: i64,
    #[serde(rename = "endTime")]
    end_time: i64,
    category: String,
    #[serde(rename = "actionType")]
    action_type: String,
    votes: i64,
    views: i64,
    locked: i64,
    hidden: i64,
    #[serde(rename = "shadowHidden")]
    shadow_hidden: i64,
    #[serde(rename = "userID")]
    user_id: String,
    description: String
}