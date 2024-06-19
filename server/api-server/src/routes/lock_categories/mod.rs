// SPDX-License-Identifier: AGPL-3.0-only
mod delete_id;
mod get_hash;
mod get_id;
mod post_lock;
use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_id::get_id))
        .route("/", post(post_lock::post_lock_categories))
        .route("/", delete(delete_id::delete_category_lock))
        .route("/:sha256HashPrefix", get(get_hash::get_hash))
}
