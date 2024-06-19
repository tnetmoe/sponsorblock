// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/isUserVip`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::is_user_vip::{IsUserVipRequestQuery, IsUserVipResponseBody};

pub async fn get_is_user_vip(Query(_query): Query<IsUserVipRequestQuery>) -> Result<Json<IsUserVipResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}