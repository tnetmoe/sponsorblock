// SPDX-License-Identifier: AGPL-3.0-only
mod get_id;
mod get_hash;
mod post_lock;
mod delete_id;
use axum::{
    routing::{get, post, delete},
    Router
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_id::get_id))
        .route("/", post(post_lock::post_lock_categories))
        .route("/", delete(delete_id::delete_category_lock))
        .route("/:sha256HashPrefix", get(get_hash::get_hash))
}