use axum::routing::get;
use axum::Router;

pub async fn health_route() -> &'static str {
    "health"
}

pub fn health_routes() -> Router {
    Router::new().route("/", get(health_route))
}
