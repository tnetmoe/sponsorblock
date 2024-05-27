// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/skipSegments/:sha256HashPrefix`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/skipSegments/:sha256HashPrefix
use serde::{Serialize, Deserialize};
use super::get::{Category, RequiredSegment, ActionType, SkipSegmentResponse};

/// url params
#[derive(Serialize, Deserialize)]
pub struct SkipSegmentsHashRequestQuery {
    pub prefix: String,
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
pub struct SkipSegmentsHashResponseBody {
    #[serde(rename = "videoID")]
    pub video_id: String,
    pub segments: Vec<SkipSegmentResponse>
}