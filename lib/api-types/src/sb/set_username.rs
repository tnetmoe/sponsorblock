// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/setUsername`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/setUsername
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct SetUsername {
    #[serde(rename = "userID")]
    user_id: String,
    username: Option<String>,
    #[serde(rename = "adminUserID")]
    admin_user_id: Option<String>
}