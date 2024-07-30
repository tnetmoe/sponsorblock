// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/skipSegments`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::skip_segments::post::{
    SkipSegmentsRequestBody, SkipSegmentsRequestQuery, SkipSegmentsResponseBody,
};

/// The path has two possible inputs: query OR body
/// This fn prefers the post body over the query params (due to the opinionated impl of the guy that wrote it (hi :D)).
/// If this breaks anything, please tell me.
pub async fn post_create(
    query: Option<Query<SkipSegmentsRequestQuery>>,
    body: Option<Json<SkipSegmentsRequestBody>>,
) -> Result<Json<SkipSegmentsResponseBody>, StatusCode> {
    match (query, body) {
        (Some(query), None) => handle_create_query(query.0).await,
        (None, Some(body)) => handle_create_body(body.0).await,
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn handle_create_query(
    _query: SkipSegmentsRequestQuery,
) -> Result<Json<SkipSegmentsResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn handle_create_body(
    _body: SkipSegmentsRequestBody,
) -> Result<Json<SkipSegmentsResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
