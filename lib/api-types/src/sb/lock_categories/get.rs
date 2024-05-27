// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/lockCategories`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/lockCategories
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(rename = "actionTypes")]
    action_types: Vec<String>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesResponse {
    categories: Vec<String>,
    reason: String,
    #[serde(rename = "actionTypes")]
    action_types: Vec<String>
}