// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/lockCategories/:sha256HashPrefix`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::lock_categories::get_hash::{
    LockCategoriesHashRequestQuery, LockCategoriesHashResponseBody,
};

pub async fn get_hash(
    Query(_query): Query<LockCategoriesHashRequestQuery>,
) -> Result<Json<LockCategoriesHashResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
