// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/addUserAsTempVIP`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/addUserAsTempVIP
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct AddUserAsTempVIPRequest {
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "adminUserID")]
    admin_user_id: String,
    #[serde(rename = "channelVideoID")]
    channel_video_id: String,
    enabled: String
    
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct AddUserAsTempVIPResponse(String, String);