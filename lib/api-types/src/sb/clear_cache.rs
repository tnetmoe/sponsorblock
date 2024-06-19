// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/clearCache`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/clearCache
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct ClearCacheRequestBody {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "videoID")]
    pub video_id: String
}

/// payload
/// while documented as:
/// ```json
/// {
///    String
/// }
/// ```
/// it's actually:
/// ```json
/// {
///   "message": "String"
/// }
/// ```
/// https://github.com/ajayyy/SponsorBlockServer/blob/a181d52fb2d9e04399510c3f5856ed0072acce09/src/routes/postClearCache.ts#L53
#[derive(Serialize, Deserialize)]
pub struct ClearCacheResponseBody {
    message: String
}