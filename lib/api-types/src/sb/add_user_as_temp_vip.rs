// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/addUserAsTempVIP`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/addUserAsTempVIP
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct AddUserAsTempVIPRequestQuery {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "adminUserID")]
    pub admin_user_id: String,
    #[serde(rename = "channelVideoID")]
    pub channel_video_id: String,
    pub enabled: String
    
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct AddUserAsTempVIPResponseBody(String, String);