// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/viewedVideoSponsorTime`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::sb::viewed_video_sponsor_time::ViewedVideoSponsorTimeRequestQuery;

pub async fn post_view(Query(_query): Query<ViewedVideoSponsorTimeRequestQuery>) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}