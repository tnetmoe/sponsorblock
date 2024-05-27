// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getViewsForUser`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getViewsForUser
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetViewsForUserRequestQuery {
    #[serde(rename = "userID")]
    pub user_id: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetViewsForUserResponseBody {
    #[serde(rename = "viewCount")]
    pub view_count: i64
}