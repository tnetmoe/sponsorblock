// SPDX-License-Identifier: AGPL-3.0-only
mod get_all;
mod get_value;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all::get_status))
        .route("/:value", get(get_value::get_status_value))
}
