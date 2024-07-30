// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/skipSegments`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::skip_segments::get::{SkipSegmentsRequestQuery, SkipSegmentsResponseBody};

pub async fn get_id(
    Query(_params): Query<SkipSegmentsRequestQuery>,
) -> Result<Json<SkipSegmentsResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
