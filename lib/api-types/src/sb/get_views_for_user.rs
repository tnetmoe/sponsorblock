// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getViewsForUser`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getViewsForUser
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetViewsForUser {
    #[serde(rename = "userID")]
    user_id: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetViewsForUserResponse {
    #[serde(rename = "viewCount")]
    view_count: i64
}