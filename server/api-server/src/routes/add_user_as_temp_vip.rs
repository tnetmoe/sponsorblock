// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/addUserAsTempVip`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::add_user_as_temp_vip::{
    AddUserAsTempVIPRequestQuery, AddUserAsTempVIPResponseBody,
};

pub async fn post_add_user_as_temp_vip(
    Query(_query): Query<AddUserAsTempVIPRequestQuery>,
) -> Result<Json<AddUserAsTempVIPResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
