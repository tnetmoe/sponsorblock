// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getVideoSponsorTimes`
//! https://wiki.sponsor.ajay.app/w/API_Docs#Legacy_API
//! https://github.com/ajayyy/SponsorBlock/wiki/Legacy-API
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetVideoSponsorTimesRequestQuery {
    #[serde(rename = "videoID")]
    pub video_id: String,
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetVideoSponsorTimesResponseBody {
    #[serde(rename = "sponsorTimes")]
    pub sponsor_times: Vec<SponsorTimes>,
    #[serde(rename = "UUIDs")]
    pub segments: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct SponsorTimes {
    #[serde(rename = "startTime")]
    pub start_time: f64,
    #[serde(rename = "endTime ")]
    pub end_time: f64,
}