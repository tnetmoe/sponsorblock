// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/getViewsForUser`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::sb::get_views_for_user::{GetViewsForUserRequestQuery, GetViewsForUserResponseBody};

pub async fn get_user_views(Query(query): Query<GetViewsForUserRequestQuery>) -> Result<Json<GetViewsForUserResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}