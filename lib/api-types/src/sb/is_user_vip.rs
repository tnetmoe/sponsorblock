// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/isUserVIP`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/isUserVIP
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct IsUserVipRequestQuery {
    #[serde(rename = "userID")]
    pub user_id: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct IsUserVipResponseBody {
    #[serde(rename = "hashedUserID")]
    pub hashed_user_id: String,
    pub vip: bool
}