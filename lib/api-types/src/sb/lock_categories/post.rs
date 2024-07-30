// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/lockCategories`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/lockCategories
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesRequestBody {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "actionTypes")]
    pub categories: Vec<String>,
    pub reason: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesResponseBody {
    pub submitted: Vec<String>,
    #[serde(rename = "submittedValues")]
    pub submitted_values: Vec<SubmittedValues>
}

#[derive(Serialize, Deserialize)]
pub struct SubmittedValues {
    #[serde(rename = "actionType")]
    pub action_type: String,
    pub category: String
}