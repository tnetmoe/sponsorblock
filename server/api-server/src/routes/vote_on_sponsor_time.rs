// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/voteOnSponsorTime`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::vote_on_sponsor_time::VoteOnSponsorTimeRequestQuery;

/// The path has two possible query inputs: normal OR category
pub async fn post_vote(
    Query(query): Query<VoteOnSponsorTimeRequestQuery>,
) -> Result<Json<()>, StatusCode> {
    match query {
        VoteOnSponsorTimeRequestQuery::Normal(_) => handle_normal_vote(query).await,
        VoteOnSponsorTimeRequestQuery::Category(_) => handle_category_vote(query).await,
    }
}

async fn handle_normal_vote(_query: VoteOnSponsorTimeRequestQuery) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn handle_category_vote(
    _query: VoteOnSponsorTimeRequestQuery,
) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
