// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/setUsername`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/setUsername
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct SetUsername {
    #[serde(rename = "userID")]
    pub user_id: String,
    pub username: Option<String>,
    #[serde(rename = "adminUserID")]
    pub admin_user_id: Option<String>
}