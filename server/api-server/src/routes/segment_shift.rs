// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/segmentShift`
use axum::{extract::Json, http::StatusCode};
use sb_api_types::sb::segment_shift::SegmentShiftRequestBody;

pub async fn post_segment_shift(
    Json(_query): Json<SegmentShiftRequestBody>,
) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
