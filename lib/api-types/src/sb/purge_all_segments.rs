// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/purgeAllSegments`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/purgeAllSegments
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct PurgeAllSegmentsRequest {
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "videoID")]
    video_id: String,
    service: String
}