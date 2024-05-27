// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getTotalStats`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getTotalStats
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetTotalStatsRequest {
    #[serde(rename = "countContributingUsers")]
    count_contributing_users: Option<bool>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetTotalStatsResponse {
    #[serde(rename = "userCount")]
    user_count: Option<i64>,
    #[serde(rename = "activeUsers")]
    active_users: i64,
    #[serde(rename = "apiUsers")]
    api_users: i64,
    #[serde(rename = "viewCount")]
    view_count: i64,
    #[serde(rename = "totalSubmissions")]
    total_submissions: i64,
    #[serde(rename = "minutesSaved")]
    minutes_saved: f64
}