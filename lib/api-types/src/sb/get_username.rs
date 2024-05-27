// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getUsername`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getUsername
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetUsernameRequest {
    #[serde(rename = "userID")]
    pub user_id: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetUsernameResponse {
    #[serde(rename = "userName")]
    pub username: String
}