// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/v1/getThumbnail`
//! https://wiki.sponsor.ajay.app/w/API_Docs/DeArrow#GET_/api/v1/getThumbnail
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GetThumbnailRequestQuery {
    #[serde(rename = "videoID")]
    pub video_id: String,
    pub time: i64
}