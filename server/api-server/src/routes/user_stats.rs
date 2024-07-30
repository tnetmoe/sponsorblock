// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/userStats`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::user_stats::{UserStatsRequestQuery, UserStatsResponseBody};

pub async fn get_user_stats(
    Query(_query): Query<UserStatsRequestQuery>,
) -> Result<Json<UserStatsResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
