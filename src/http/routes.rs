use super::infra::health::health_routes;
use super::ollama::config_routes::ollama_routes;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

pub fn app_routes() -> Router {
    Router::new()
        .nest("/", health_routes())
        .nest("/api", ollama_routes())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        )
}
