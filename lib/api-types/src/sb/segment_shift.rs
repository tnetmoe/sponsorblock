// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/segmentShift`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/segmentShift
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct SegmentShiftRequest {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "startTime")]
    pub start_time: f64,
    #[serde(rename = "endTime")]
    pub end_time: f64
}