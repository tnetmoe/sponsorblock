// SPDX-License-Identifier: LGPL-3.0-only
//! GET `/api/getDaysSavedFormatted`
//! https://wiki.sponsor.ajay.app/w/API_Docs#GET_/api/getDaysSavedFormatted
use serde::{Serialize, Deserialize};

/// payload
#[derive(Serialize, Deserialize)]
pub struct GetDaysSavedFormattedResponse {
    #[serde(rename = "daysSaved")]
    days_saved: f64
}