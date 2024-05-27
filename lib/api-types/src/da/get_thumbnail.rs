// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/v1/getThumbnail`
//! https://wiki.sponsor.ajay.app/w/API_Docs/DeArrow#GET_/api/v1/getThumbnail
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetThumbnailRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    time: i64
}