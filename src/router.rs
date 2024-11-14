use axum::routing::get;
use axum::Router;

pub fn initialize_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
}
