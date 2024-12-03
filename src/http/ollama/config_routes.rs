use super::chat_handler::chat_stream;
use super::delete_model_handler::delete_stream;
use super::generate_handler::generate_stream;
use super::list_models_handler::tags;
use super::pull_model_handler::pull_stream;
use super::running_models_handler::ps;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

pub fn ollama_routes() -> Router {
    Router::new()
        .route("/generate", post(generate_stream))
        .route("/chat", post(chat_stream))
        .route("/pull", post(pull_stream))
        .route("/delete", delete(delete_stream))
        .route("/tags", get(tags))
        .route("/ps", get(ps))
}
