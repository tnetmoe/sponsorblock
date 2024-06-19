// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/addUserAsVIP`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/addUserAsVIP
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct AddUserAsVIPRequestQuery {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "adminUserID")]
    pub admin_user_id: String,
    pub enabled: Option<String>
}