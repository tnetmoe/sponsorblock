// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/branding/:sha256HashPrefix`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::da::branding::get_hash::{BrandingHashRequestQuery, BrandingHashResponseBody};

pub async fn get_hash(Query(_query): Query<BrandingHashRequestQuery>) -> Result<Json<BrandingHashResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}