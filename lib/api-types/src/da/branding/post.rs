// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/branding`
//! https://wiki.sponsor.ajay.app/w/API_Docs/DeArrow#POST_/api/branding
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct BrandingRequest {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub service: Option<String>,
    pub title: Option<Title>,
    pub thumbnail: Option<Thumbnail>,
    pub downvote: Option<bool>,
    #[serde(rename = "autoLock")]
    pub auto_lock: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnail {
    pub timestamp: Option<i64>,
    pub original: bool
}