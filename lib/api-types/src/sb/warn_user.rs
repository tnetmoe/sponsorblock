// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/warnUser`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/warnUser
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct WarnUserRequest {
    #[serde(rename = "issuerUserID")]
    issuer_user_id: String,
    #[serde(rename = "userID")]
    user_id: String,
    reason: Option<String>,
    enabled: Option<bool>
}