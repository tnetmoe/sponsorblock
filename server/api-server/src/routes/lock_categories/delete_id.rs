// SPDX-License-Identifier: AGPL-3.0-only
//! DELETE `/api/lockCategories`
use axum::{
    extract::Json,
    http::StatusCode
};
use sb_api_types::sb::lock_categories::delete::{LockCategoriesRequestBody, LockCategoriesResponseBody};

pub async fn delete_category_lock(Json(_body): Json<LockCategoriesRequestBody>) -> Result<Json<LockCategoriesResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}