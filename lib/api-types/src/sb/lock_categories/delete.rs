// SPDX-License-Identifier: LGPL-3.0-only
//! DELETE `/api/lockCategories`
//! https://wiki.sponsor.ajay.app/w/API_Docs#DELETE_/api/lockCategories
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesRequestBody {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    pub categories: Vec<String>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesResponseBody {
    pub message: String
}