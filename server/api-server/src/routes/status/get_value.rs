// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/status/:value`
use axum::{
    extract::{Path, Json},
    http::StatusCode,
};
use sb_api_types::sb::status::StatusResponseBody;

pub async fn get_status_value(Path(_value): Path<String>) -> Result<Json<StatusResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}