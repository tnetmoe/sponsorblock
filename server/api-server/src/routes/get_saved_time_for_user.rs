// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/getSavedTimeForUser`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::sb::get_saved_time_for_user::{GetSavedTimeForUserRequestQuery, GetSavedTimeForUserResponseBody};

pub async fn get_user_saved_time(Query(query): Query<GetSavedTimeForUserRequestQuery>) -> Result<Json<GetSavedTimeForUserResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}