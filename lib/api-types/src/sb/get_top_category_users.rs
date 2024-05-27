// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getTopCategoryUsers`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getTopCategoryUsers
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetTopCategoryUsersRequestQuery {
    #[serde(rename = "sortType")]
    pub time_frame: u8,
    pub category: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetTopCategoryUsersResponseBody {
    #[serde(rename = "userNames")]
    pub user_names: Vec<String>,
    #[serde(rename = "viewCounts")]
    pub view_counts: Vec<i64>,
    #[serde(rename = "totalSubmissions")]
    pub total_submissions: Vec<i64>,
    #[serde(rename = "minutesSaved")]
    pub minutes_saved: Vec<f64>
}