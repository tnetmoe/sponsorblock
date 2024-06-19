// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/shadowBanUser`
use axum::{
    extract::{Json, Query},
    http::StatusCode,
};
use sb_api_types::sb::shadow_ban_user::ShadowBanUserRequestQuery;

pub async fn post_shadow_ban_user(
    Query(_query): Query<ShadowBanUserRequestQuery>,
) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
