// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/purgeAllSegments`
use axum::{extract::Json, http::StatusCode};
use sb_api_types::sb::purge_all_segments::PurgeAllSegmentsRequestBody;

pub async fn post_purge_all_segments(
    Json(_body): Json<PurgeAllSegmentsRequestBody>,
) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
