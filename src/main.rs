use axum::{handler::Handler, routing};

#[tokio::main]
async fn main() {
    let fizz = axum::Router::new()
        .route("/buzz", routing::get(|| async { "FizzBuzz" }))
        .fallback(fizz_fallback.into_service());

    let router = axum::Router::new()
        .route("/foo", routing::get(|| async { "Foo" }))
        .route("/bar", routing::get(|| async { "Bar" }))
        .nest("/fizz", routing::service_method_routing::any(fizz))
        .fallback(top_level_fallback.into_service());

    axum::Server::bind(&"0.0.0.0:3333".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn top_level_fallback() -> impl axum::response::IntoResponse {
    "Top level fallback"
}

async fn fizz_fallback() -> impl axum::response::IntoResponse {
    "Fizz fallback"
}
