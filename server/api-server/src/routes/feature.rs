// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/feature`
use axum::{
    extract::Json,
    http::StatusCode,
};
use sb_api_types::sb::feature::FeatureRequestBody;

pub async fn post_feature(Json(body): Json<FeatureRequestBody>) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}