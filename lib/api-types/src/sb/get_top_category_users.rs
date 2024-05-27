// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getTopCategoryUsers`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getTopCategoryUsers
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetTopCategoryUsersRequest {
    #[serde(rename = "sortType")]
    time_frame: u8,
    category: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetTopCategoryUsersResponse {
    #[serde(rename = "userNames")]
    user_names: Vec<String>,
    #[serde(rename = "viewCounts")]
    view_counts: Vec<i64>,
    #[serde(rename = "totalSubmissions")]
    total_submissions: Vec<i64>,
    #[serde(rename = "minutesSaved")]
    minutes_saved: Vec<f64>
}