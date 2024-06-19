// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/setUsername`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::sb::set_username::SetUsernameRequestQuery;

pub async fn post_set_username(Query(_query): Query<SetUsernameRequestQuery>) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}