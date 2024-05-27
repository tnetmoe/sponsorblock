// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/userID`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/userID
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct UserIDRequest {
    username: String,
    exact: bool
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct UserIDResponse(Vec<UserSearch>);

#[derive(Serialize, Deserialize)]
pub struct UserSearch {
    #[serde(rename = "userName")]
    username: String,
    #[serde(rename = "userID")]
    user_id: String
}