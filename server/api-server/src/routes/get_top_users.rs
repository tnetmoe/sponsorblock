// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/getTopUsers`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::get_top_users::{GetTopUsersRequestQuery, GetTopUsersResponseBody};

pub async fn get_top_users(
    Query(_query): Query<GetTopUsersRequestQuery>,
) -> Result<Json<GetTopUsersResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
