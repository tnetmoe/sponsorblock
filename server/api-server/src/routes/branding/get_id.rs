// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/branding`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::da::branding::get::{BrandingRequestQuery, BrandingResponseBody};

pub async fn get_id(
    Query(_query): Query<BrandingRequestQuery>,
) -> Result<Json<BrandingResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
