// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/getDaysSavedFormatted`
use axum::{
    extract::Json,
    http::StatusCode,
};
use sb_api_types::sb::get_days_saved_formatted::GetDaysSavedFormattedResponseBody;

pub async fn get_time_saved_stats() -> Result<Json<GetDaysSavedFormattedResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}