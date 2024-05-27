// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/lockCategories`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/lockCategories
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "actionTypes")]
    categories: Vec<String>,
    reason: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockCategoriesResponse {
    submitted: Vec<String>,
    #[serde(rename = "submittedValues")]
    submitted_values: Vec<SubmittedValues>
}

#[derive(Serialize, Deserialize)]
pub struct SubmittedValues {
    #[serde(rename = "actionType")]
    action_type: String,
    category: String
}