// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/userStats`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/userStats
use serde::{Serialize, Deserialize};
use super::user_info::UserId;

/// url params
#[derive(Serialize, Deserialize)]
pub struct UserStatsRequestQuery {
    #[serde(flatten)]
    pub user_id: UserId,
    #[serde(rename = "fetchCategoryStats")]
    pub fetch_category_stats: bool,
    #[serde(rename = "fetchActionTypeStats")]
    pub fetch_action_type_stats: bool
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct UserStatsResponseBody {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: f64,
    #[serde(rename = "overallStats")]
    pub overall_stats: OverallStats,
    #[serde(rename = "categoryCount")]
    pub category_count: Option<CategoryCount>,
    #[serde(rename = "actionTypeCount")]
    pub action_type_count: Option<ActionTypeCount>,
}

#[derive(Serialize, Deserialize)]
pub struct OverallStats {
    #[serde(rename = "minutesSaved")]
    pub minutes_saved: f64,
    #[serde(rename = "segmentCount")]
    pub segment_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryCount {
    pub sponsor: i64,
    pub intro: i64,
    pub outro: i64,
    pub interaction: i64,
    pub selfpromo: i64,
    pub music_offtopic: i64,
    pub preview: i64,
    pub poi_highlight: i64,
    pub filler: i64,
    pub exclusive_access: i64,
    pub chapter: i64
}

#[derive(Serialize, Deserialize)]
pub struct ActionTypeCount {
    pub skip: i64,
    pub mute: i64,
    pub full: i64,
    pub poi: i64,
    pub chapter: i64
}