// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/addUserAsVip`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::add_user_as_vip::AddUserAsVIPRequestQuery;

pub async fn post_add_user_as_vip(Query(_query): Query<AddUserAsVIPRequestQuery>) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}