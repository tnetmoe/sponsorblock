// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/feature`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/feature
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct FeatureRequest {
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "adminUserID")]
    admin_user_id: String,
    feature: i64,
    enabled: bool
}