// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/skipSegments/:sha256HashPrefix`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/skipSegments/:sha256HashPrefix
use serde::{Serialize, Deserialize};
use super::get::{Category, RequiredSegment, ActionType, SkipSegmentResponse};

/// url params
#[derive(Serialize, Deserialize)]
pub struct SkipSegmentsHashRequest {
    prefix: String,
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
pub struct SkipSegmentsHashResponse {
    #[serde(rename = "videoID")]
    video_id: String,
    segments: Vec<SkipSegmentResponse>
}