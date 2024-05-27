// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/userInfo`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/userInfo
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct UserInfoRequest {
    #[serde(flatten)]
    pub user_id: UserId,
    #[serde(flatten)]
    pub value: Value
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct UserInfoResponse {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "minutesSaved")]
    pub minutes_saved: f64,
    #[serde(rename = "segmentCount")]
    pub segment_count: i64,
    #[serde(rename = "ignoredSegmentCount")]
    pub ignored_segment_count: i64,
    #[serde(rename = "viewCount")]
    pub view_count: i64,
    #[serde(rename = "ignoredViewCount")]
    pub ignored_view_count: i64,
    pub warnings: i64,
    pub reputation: i64,
    pub vip: i64,
    #[serde(rename = "lastSegmentID")]
    pub last_segment_id: String,
    pub permissions: HashMap<String, bool>
}

#[derive(Serialize, Deserialize)]
pub enum UserId {
    #[serde(rename = "userID")]
    UserID(String),
    #[serde(rename = "publicUserID")]
    PublicUserID(String)
}

#[derive(Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "value")]
    Value(String),
    #[serde(rename = "values")]
    Values(Vec<String>)
}