// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/getSavedTimeForUser`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::get_saved_time_for_user::{
    GetSavedTimeForUserRequestQuery, GetSavedTimeForUserResponseBody,
};

pub async fn get_user_saved_time(
    Query(_query): Query<GetSavedTimeForUserRequestQuery>,
) -> Result<Json<GetSavedTimeForUserResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
