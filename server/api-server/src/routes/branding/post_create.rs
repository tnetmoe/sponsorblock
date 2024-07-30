// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/branding`
use axum::{extract::Json, http::StatusCode};
use sb_api_types::da::branding::post::BrandingRequestBody;

pub async fn post_create(Json(_body): Json<BrandingRequestBody>) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
