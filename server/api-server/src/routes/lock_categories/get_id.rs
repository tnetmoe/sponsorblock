// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/lockCategories`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::lock_categories::get::{
    LockCategoriesRequestQuery, LockCategoriesResponseBody,
};

pub async fn get_id(
    Query(_query): Query<LockCategoriesRequestQuery>,
) -> Result<Json<LockCategoriesResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
