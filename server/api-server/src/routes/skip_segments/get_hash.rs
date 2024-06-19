// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/skipSegments/:sha256HashPrefix`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::sb::skip_segments::get_hash::{SkipSegmentsHashRequestQuery, SkipSegmentsHashResponseBody};

pub async fn get_hash(Query(_query): Query<SkipSegmentsHashRequestQuery>) -> Result<Json<SkipSegmentsHashResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}