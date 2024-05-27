// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/shadowBanUser`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/shadowBanUser
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct ShadowBanUserRequest {
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "adminUserID")]
    admin_user_id: String,
    #[serde(rename = "unHideOldSubmissions")]
    un_hide_old_submissions: bool,
    categories: Option<String>
}