// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getVideoSponsorTimes`
//! https://wiki.sponsor.ajay.app/w/API_Docs#Legacy_API
//! https://github.com/ajayyy/SponsorBlock/wiki/Legacy-API
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetVideoSponsorTimesRequest {
    #[serde(rename = "videoID")]
    video_id: String,
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetVideoSponsorTimesResponse {
    #[serde(rename = "sponsorTimes")]
    sponsor_times: Vec<SponsorTimes>,
    #[serde(rename = "UUIDs")]
    segments: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct SponsorTimes {
    #[serde(rename = "startTime")]
    start_time: f64,
    #[serde(rename = "endTime ")]
    end_time: f64,
}