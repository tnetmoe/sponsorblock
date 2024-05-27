// SPDX-License-Identifier: LGPL-3.0-only
//! POST `/api/viewedVideoSponsorTime`
//! https://wiki.sponsor.ajay.app/w/API_Docs#POST_/api/viewedVideoSponsorTime
use serde::{Serialize, Deserialize};

/// url params
#[derive(Serialize, Deserialize)]
pub struct ViewedVideoSponsorTimeRequestQuery {
    #[serde(rename = "UUID")]
    pub uuid: String
}