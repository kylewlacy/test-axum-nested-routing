use std::path::PathBuf;

use axum::{error_handling::HandleErrorExt, handler::Handler, http::StatusCode, routing};

#[tokio::main]
async fn main() {
    let path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static"));

    let swagger = axum::Router::new()
        .route("/schema.json", routing::get(|| async { "schema.json" }))
        .fallback(
            tower_http::services::ServeDir::new(path)
                .handle_error(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")),
        );

    let router = axum::Router::new()
        .nest("/v1/swagger", routing::service_method_routing::any(swagger))
        .fallback(fallback_handler.into_service());

    axum::Server::bind(&"0.0.0.0:3333".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn fallback_handler() -> impl axum::response::IntoResponse {
    "Top level fallback"
}
