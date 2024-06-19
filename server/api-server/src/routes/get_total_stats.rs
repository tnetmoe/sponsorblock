// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/getTotalStats`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::get_total_stats::{GetTotalStatsRequestQuery, GetTotalStatsResponseBody};

pub async fn get_total_stats(Query(query): Query<GetTotalStatsRequestQuery>) -> Result<Json<GetTotalStatsResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}