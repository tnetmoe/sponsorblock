// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/isUserVIP`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/isUserVIP
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct IsUserVipRequest {
    #[serde(rename = "userID")]
    user_id: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct IsUserVipResponse {
    #[serde(rename = "hashedUserID")]
    hashed_user_id: String,
    vip: bool
}