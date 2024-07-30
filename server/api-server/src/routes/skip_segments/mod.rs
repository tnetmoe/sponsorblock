// SPDX-License-Identifier: AGPL-3.0-only
mod get_hash;
mod get_id;
mod post_create;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_id::get_id))
        .route("/", post(post_create::post_create))
        .route("/:sha256HashPrefix", get(get_hash::get_hash))
}
