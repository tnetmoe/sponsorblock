// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/searchSegments`
use axum::{
    body, extract::{Json, Query}, http::StatusCode
};
use sb_api_types::sb::search_segments::{SearchSegmentsRequest, SearchSegmentsResponseBody};

/// For whatever insane reason, this GET endpoint accepts both query parameters OR a (fricking) JSON body.
/// The guy who wrote the code (Hi :D) has, for obvious reasons, created an opinionated implementation that prefers the query params over the body.
pub async fn get_search_segments(
    query: Option<Query<SearchSegmentsRequest>>,
    body: Option<Json<SearchSegmentsRequest>>
) -> Result<Json<SearchSegmentsResponseBody>, StatusCode> {
    match (query, body) {
        (Some(query), None) => handle_query(query.0).await,
        (None, Some(body)) => handle_body(body.0).await,
        _ => Err(StatusCode::BAD_REQUEST)
    }
}

async fn handle_query(query: SearchSegmentsRequest) -> Result<Json<SearchSegmentsResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn handle_body(body: SearchSegmentsRequest) -> Result<Json<SearchSegmentsResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}