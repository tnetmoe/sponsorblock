// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/userStats`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::sb::user_stats::{UserStatsRequestQuery, UserStatsResponseBody};

pub async fn get_user_stats(Query(query): Query<UserStatsRequestQuery>) -> Result<Json<UserStatsResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}