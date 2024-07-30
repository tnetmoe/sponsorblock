// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/lockReason`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/lockReason
use serde::{Serialize, Deserialize};
use super::skip_segments::get::Category;

/// url params
#[derive(Serialize, Deserialize)]
pub struct LockReasonRequestQuery {
    #[serde(rename = "videoID")]
    pub video_id: String,
    #[serde(flatten)]
    pub category: Category,
    #[serde(rename = "actionTypes")]
    pub action_types: Vec<String>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockReasonResponseBody(Vec<LockReason>);

#[derive(Serialize, Deserialize)]
pub struct LockReason {
    pub category: String,
    pub locked: i64,
    pub reason: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub username: String
}