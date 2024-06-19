// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/clearCache`
use axum::{
    extract::Json,
    http::StatusCode,
};
use sb_api_types::sb::clear_cache::ClearCacheRequestBody;

pub async fn post_clear_cache(Json(body): Json<ClearCacheRequestBody>) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}