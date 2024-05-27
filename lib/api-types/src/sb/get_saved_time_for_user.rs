// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/getSavedTimeForUser`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getSavedTimeForUser
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetSavedTimeForUser {
    #[serde(rename = "userID")]
    user_id: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetSavedTimeForUserResponse {
    #[serde(rename = "timeSaved")]
    time_saved: f64
}