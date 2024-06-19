// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/branding`
use axum::{
    http::StatusCode,
    extract::{Path, Query, Json}
};
use sb_api_types::da::branding::get::{BrandingRequestQuery, BrandingResponseBody};

pub async fn get_id(Query(query): Query<BrandingRequestQuery>) -> Result<Json<BrandingResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}