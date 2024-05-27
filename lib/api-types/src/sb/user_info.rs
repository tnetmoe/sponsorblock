// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/userInfo`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/userInfo
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct UserInfoRequest {
    #[serde(flatten)]
    user_id: UserId,
    #[serde(flatten)]
    value: Value
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct UserInfoResponse {
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "userName")]
    user_name: String,
    #[serde(rename = "minutesSaved")]
    minutes_saved: f64,
    #[serde(rename = "segmentCount")]
    segment_count: i64,
    #[serde(rename = "ignoredSegmentCount")]
    ignored_segment_count: i64,
    #[serde(rename = "viewCount")]
    view_count: i64,
    #[serde(rename = "ignoredViewCount")]
    ignored_view_count: i64,
    warnings: i64,
    reputation: i64,
    vip: i64,
    #[serde(rename = "lastSegmentID")]
    last_segment_id: String,
    permissions: HashMap<String, bool>
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