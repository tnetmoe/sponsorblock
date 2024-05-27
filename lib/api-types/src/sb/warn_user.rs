// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/warnUser`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/warnUser
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct WarnUserRequestBody {
    #[serde(rename = "issuerUserID")]
    pub issuer_user_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
    pub reason: Option<String>,
    pub enabled: Option<bool>
}