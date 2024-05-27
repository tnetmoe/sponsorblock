// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/addUserAsVIP`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/addUserAsVIP
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct AddUserAsVIPRequest {
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "adminUserID")]
    admin_user_id: String,
    enabled: String
}