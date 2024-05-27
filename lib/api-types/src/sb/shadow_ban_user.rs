// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/shadowBanUser`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/shadowBanUser
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct ShadowBanUserRequestQuery {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "adminUserID")]
    pub admin_user_id: String,
    #[serde(rename = "unHideOldSubmissions")]
    pub un_hide_old_submissions: bool,
    pub categories: Option<String>
}