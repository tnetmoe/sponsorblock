// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getTotalStats`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getTotalStats
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetTotalStatsRequestQuery {
    #[serde(rename = "countContributingUsers")]
    pub count_contributing_users: Option<bool>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetTotalStatsResponseBody {
    #[serde(rename = "userCount")]
    pub user_count: Option<i64>,
    #[serde(rename = "activeUsers")]
    pub active_users: i64,
    #[serde(rename = "apiUsers")]
    pub api_users: i64,
    #[serde(rename = "viewCount")]
    pub view_count: i64,
    #[serde(rename = "totalSubmissions")]
    pub total_submissions: i64,
    #[serde(rename = "minutesSaved")]
    pub minutes_saved: f64
}