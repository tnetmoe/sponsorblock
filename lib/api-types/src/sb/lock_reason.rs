// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/lockReason`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/lockReason
use serde::{Serialize, Deserialize};
use super::skip_segments::get::Category;

/// url params
#[derive(Serialize, Deserialize)]
pub struct LockReasonRequest {
    #[serde(rename = "videoID")]
    video_id: String,
    #[serde(flatten)]
    category: Category,
    #[serde(rename = "actionTypes")]
    action_types: Vec<String>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct LockReasonResponse(Vec<LockReason>);

#[derive(Serialize, Deserialize)]
pub struct LockReason {
    category: String,
    locked: i64,
    reason: String,
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "userName")]
    username: String
}