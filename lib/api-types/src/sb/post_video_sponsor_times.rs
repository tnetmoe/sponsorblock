// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/postVideoSponsorTimes`
//! https://wiki.sponsor.ajay.app/w/API_Docs#Legacy_API
//! https://github.com/ajayyy/SponsorBlock/wiki/Legacy-API
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct PostVideoSponsorTimesRequest {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(rename = "startTime")]
    pub start_time: Option<f64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<f64>,
    #[serde(rename = "userID")]
    pub user_id: Option<String>
}