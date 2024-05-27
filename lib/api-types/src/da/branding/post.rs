// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/branding`
//! https://wiki.sponsor.ajay.app/w/API_Docs/DeArrow#POST_/api/branding
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct BrandingRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
    service: Option<String>,
    title: Option<Title>,
    thumbnail: Option<Thumbnail>,
    downvote: Option<bool>,
    #[serde(rename = "autoLock")]
    auto_lock: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    title: String
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnail {
    timestamp: Option<i64>,
    original: bool
}