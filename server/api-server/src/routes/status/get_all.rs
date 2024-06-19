// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/status`
use axum::{extract::Json, http::StatusCode};

use sb_api_types::sb::status::StatusResponseBody;

pub async fn get_status() -> Result<Json<StatusResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
