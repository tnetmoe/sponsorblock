// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/postVideoSponsorTimes`
//! https://wiki.sponsor.ajay.app/w/API_Docs#Legacy_API
//! https://github.com/ajayyy/SponsorBlock/wiki/Legacy-API
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct PostVideoSponsorTimesRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(rename = "startTime")]
    start_time: Option<f64>,
    #[serde(rename = "endTime")]
    end_time: Option<f64>,
    #[serde(rename = "userID")]
    user_id: Option<String>
}