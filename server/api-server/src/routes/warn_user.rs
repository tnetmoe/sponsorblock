// SPDX-License-Identifier: AGPL-3.0-only
//! POST `/api/warnUser`
use axum::{extract::Json, http::StatusCode};
use sb_api_types::sb::warn_user::WarnUserRequestBody;

pub async fn post_warn_user(
    Json(_body): Json<WarnUserRequestBody>,
) -> Result<Json<()>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
