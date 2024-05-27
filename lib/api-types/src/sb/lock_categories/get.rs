// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/lockCategories`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/lockCategories
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesRequest {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(rename = "actionTypes")]
    pub action_types: Vec<String>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesResponse {
    pub categories: Vec<String>,
    pub reason: String,
    #[serde(rename = "actionTypes")]
    pub action_types: Vec<String>
}