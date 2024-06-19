// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/userID`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::sb::user_id::{UserIDRequestQuery, UserIDResponseBody};

pub async fn get_user_name_search(Query(_query): Query<UserIDRequestQuery>) -> Result<Json<UserIDResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}