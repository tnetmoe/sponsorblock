// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/clearCache`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/clearCache
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct ClearCacheRequest {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "videoID")]
    pub video_id: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct ClearCacheResponse(String);