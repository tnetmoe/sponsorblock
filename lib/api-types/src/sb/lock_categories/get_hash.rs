// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/lockCategories/:sha256HashPrefix`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/lockCategories/:sha256HashPrefix
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesHashRequestQuery {
    pub prefix: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesHashResponseBody(Vec<LockCategorieHash>);

#[derive(Serialize, Deserialize)]
pub struct LockCategorieHash {
    #[serde(rename = "videoID")]
    pub video_id: String,
    pub hash: String,
    pub categories: Vec<String>,
    pub reason: String
}