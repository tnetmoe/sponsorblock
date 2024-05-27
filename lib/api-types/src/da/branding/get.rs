// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/branding`
//! https://wiki.sponsor.ajay.app/w/API_Docs/DeArrow#GET_/api/branding
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct BrandingRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    service: Option<String>,
    #[serde(rename = "returnUserID")]
    return_user_id: Option<bool>,
    #[serde(rename = "fetchAll")]
    fetch_all: Option<bool>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct BrandingResponse {
    titles: Vec<String>,
    thumbnails: Vec<String>,
    #[serde(rename = "randomTime")]
    random_time: i64,
    #[serde(rename = "videoDuration")]
    video_duration: Option<i64>
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    title: String,
    original: String,
    votes: i64,
    #[serde(rename = "UUID")]
    uuid: String,
    #[serde(rename = "userID")]
    user_id: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnail {
    timestamp: Option<i64>,
    original: bool,
    votes: i64,
    locked: bool,
    #[serde(rename = "UUID")]
    uuid: String,
    #[serde(rename = "userID")]
    user_id: Option<String>
}