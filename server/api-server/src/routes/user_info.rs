// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/userInfo`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::sb::user_info::{UserInfoRequestQuery, UserInfoResponseBody};

pub async fn get_user_info(Query(query): Query<UserInfoRequestQuery>) -> Result<Json<UserInfoResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}