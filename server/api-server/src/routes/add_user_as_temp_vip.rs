// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/addUserAsTempVip`
use axum::{extract::Query, http::StatusCode, response::Response};
#[allow(unused_imports)]
use sb_api_types::sb::add_user_as_temp_vip::{
    AddUserAsTempVIPRequestQuery, AddUserAsTempVIPResponseBody,
};

pub async fn post_add_user_as_temp_vip(
    Query(_query): Query<AddUserAsTempVIPRequestQuery>,
) -> Result<Response, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
