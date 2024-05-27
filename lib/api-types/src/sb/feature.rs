// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/feature`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/feature
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct FeatureRequestBody {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "adminUserID")]
    pub admin_user_id: String,
    pub feature: i64,
    pub enabled: bool
}