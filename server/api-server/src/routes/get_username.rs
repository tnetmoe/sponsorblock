// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/getUsername`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::get_username::{GetUsernameRequestQuery, GetUsernameResponseBody};

pub async fn get_username(
    Query(_query): Query<GetUsernameRequestQuery>,
) -> Result<Json<GetUsernameResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
