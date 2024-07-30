// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/getViewsForUser`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::get_views_for_user::{
    GetViewsForUserRequestQuery, GetViewsForUserResponseBody,
};

pub async fn get_user_views(
    Query(_query): Query<GetViewsForUserRequestQuery>,
) -> Result<Json<GetViewsForUserResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
