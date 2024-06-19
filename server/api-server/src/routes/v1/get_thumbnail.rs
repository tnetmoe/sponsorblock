// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/v1/getThumbnail`
use axum::{extract::Query, http::StatusCode, response::Response};
use sb_api_types::da::get_thumbnail::GetThumbnailRequestQuery;

/// Example of how to return a file dynamically: https://github.com/tokio-rs/axum/discussions/608
pub async fn get_thumbnail(
    Query(_query): Query<GetThumbnailRequestQuery>,
) -> Result<Response, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
