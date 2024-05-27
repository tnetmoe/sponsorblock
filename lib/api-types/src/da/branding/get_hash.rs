// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/branding/:sha256HashPrefix`
//! https://wiki.sponsor.ajay.app/w/API_Docs/DeArrow#GET_/api/branding/:sha256HashPrefix
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::get::BrandingResponse;

/// url params
#[derive(Serialize, Deserialize)]
pub struct GetBrandingHashRequestQuery {
    pub service: Option<String>,
    #[serde(rename = "returnUserID")]
    pub return_user_id: Option<String>,
    #[serde(rename = "fetchAll")]
    pub fetch_all: Option<String>
}

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetBrandingHashResponseBody(HashMap<String, BrandingResponse>);