// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getTopUsers`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getTopUsers
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetTopUsersRequest {
    #[serde(rename = "sortType")]
    pub sort_type: u8
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetTopUsersResponse {
    #[serde(rename = "userNames")]
    pub user_names: Vec<String>,
    #[serde(rename = "viewCounts")]
    pub view_counts: Vec<i64>,
    #[serde(rename = "totalSubmissions")]
    pub total_submissions: Vec<i64>,
    #[serde(rename = "minutesSaved")]
    pub minutes_saved: Vec<f64>
}