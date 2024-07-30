// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/segmentInfo`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::segment_info::{SegmentInfoRequestQuery, SegmentInfoResponseBody};

pub async fn get_segment_info(
    Query(_query): Query<SegmentInfoRequestQuery>,
) -> Result<Json<SegmentInfoResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
