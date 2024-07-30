// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/lockCategories`
use axum::{extract::Json, http::StatusCode};
use sb_api_types::sb::lock_categories::post::{
    LockCategoriesRequestBody, LockCategoriesResponseBody,
};

pub async fn post_lock_categories(
    Json(_body): Json<LockCategoriesRequestBody>,
) -> Result<Json<LockCategoriesResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
