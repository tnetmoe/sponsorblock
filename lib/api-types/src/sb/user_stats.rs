// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/userStats`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/userStats
use serde::{Serialize, Deserialize};
use super::user_info::UserId;

/// url params
#[derive(Serialize, Deserialize)]
pub struct UserStatsRequest {
    #[serde(flatten)]
    user_id: UserId,
    #[serde(rename = "fetchCategoryStats")]
    fetch_category_stats: bool,
    #[serde(rename = "fetchActionTypeStats")]
    fetch_action_type_stats: bool
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct UserStatsResponse {
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "userName")]
    user_name: f64,
    #[serde(rename = "overallStats")]
    overall_stats: OverallStats,
    #[serde(rename = "categoryCount")]
    category_count: Option<CategoryCount>,
    #[serde(rename = "actionTypeCount")]
    action_type_count: Option<ActionTypeCount>,
}

#[derive(Serialize, Deserialize)]
pub struct OverallStats {
    #[serde(rename = "minutesSaved")]
    minutes_saved: f64,
    #[serde(rename = "segmentCount")]
    segment_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryCount {
    sponsor: i64,
    intro: i64,
    outro: i64,
    interaction: i64,
    selfpromo: i64,
    music_offtopic: i64,
    preview: i64,
    poi_highlight: i64,
    filler: i64,
    exclusive_access: i64,
    chapter: i64
}

#[derive(Serialize, Deserialize)]
pub struct ActionTypeCount {
    skip: i64,
    mute: i64,
    full: i64,
    poi: i64,
    chapter: i64
}