// SPDX-License-Identifier: AGPL-3.0-only
mod get_thumbnail;
use axum::{
    routing::get,
    Router
};

pub fn routes() -> Router {
    Router::new()
        .route("/getThumbnail", get(get_thumbnail::get_thumbnail))
}