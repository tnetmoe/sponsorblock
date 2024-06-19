// SPDX-License-Identifier: AGPL-3.0-only
//! GET `/api/lockReason`
use axum::{
    http::StatusCode,
    extract::{Query, Json}
};
use sb_api_types::sb::lock_reason::{LockReasonRequestQuery, LockReasonResponseBody};

pub async fn get_lock_reason(Query(query): Query<LockReasonRequestQuery>) -> Result<Json<LockReasonResponseBody>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}