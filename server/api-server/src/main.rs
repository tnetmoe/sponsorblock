// SPDX-License-Identifier: AGPL-3.0-only
mod routes;
use axum::Router;
use crate::routes::routes;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let router = routes();
    let app = Router::new()
        .nest("/", router);

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}