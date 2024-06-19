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
    pub admin_user_id: Option<String>,
    #[serde(rename = "channelVideoID")]
    pub channel_video_id: String,
    pub enabled: Option<String>
    
}

/// payload
/// 
/// while documented as:
/// ```json
/// {
///     String
///     String
/// }
/// ```
/// it's actually just a single plain text string
/// https://github.com/ajayyy/SponsorBlockServer/blob/a181d52fb2d9e04399510c3f5856ed0072acce09/src/routes/addUserAsTempVIP.ts#L67
#[allow(dead_code)]
pub struct AddUserAsTempVIPResponseBody(String);