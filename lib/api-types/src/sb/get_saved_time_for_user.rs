// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/getSavedTimeForUser`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getSavedTimeForUser
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetSavedTimeForUserRequestQuery {
    #[serde(rename = "userID")]
    pub user_id: String
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetSavedTimeForUserResponseBody {
    #[serde(rename = "timeSaved")]
    pub time_saved: f64
}