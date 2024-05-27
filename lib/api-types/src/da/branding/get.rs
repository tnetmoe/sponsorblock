// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/branding`
//! https://wiki.sponsor.ajay.app/w/API_Docs/DeArrow#GET_/api/branding
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct BrandingRequest {
    #[serde(rename = "videoID")]
    pub video_id: String,
    pub service: Option<String>,
    #[serde(rename = "returnUserID")]
    pub return_user_id: Option<bool>,
    #[serde(rename = "fetchAll")]
    pub fetch_all: Option<bool>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct BrandingResponse {
    pub titles: Vec<String>,
    pub thumbnails: Vec<String>,
    #[serde(rename = "randomTime")]
    pub random_time: i64,
    #[serde(rename = "videoDuration")]
    pub video_duration: Option<i64>
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    pub title: String,
    pub original: String,
    pub votes: i64,
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "userID")]
    pub user_id: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnail {
    pub timestamp: Option<i64>,
    pub original: bool,
    pub votes: i64,
    pub locked: bool,
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "userID")]
    pub user_id: Option<String>
}