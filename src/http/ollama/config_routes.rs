use super::chat_collection_handler::chat_collection;
use super::chat_handler::chat_stream;
use super::chat_pdf_handler::chat_pdf;
use super::delete_model_handler::delete_stream;
use super::embedding_handler::embedding;
use super::generate_handler::generate_stream;
use super::list_models_handler::tags;
use super::pull_model_handler::pull_stream;
use super::running_models_handler::ps;
use crate::config::environment::get_config_values;
use axum::extract::DefaultBodyLimit;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

pub fn ollama_routes() -> Router {
    let state = get_config_values();
    Router::new()
        .route("/generate", post(generate_stream))
        .route("/chat", post(chat_stream))
        .route("/pull", post(pull_stream))
        .route("/delete", delete(delete_stream))
        .route("/tags", get(tags))
        .route("/ps", get(ps))
        .route("/chat_pdf", post(chat_pdf))
        .layer(DefaultBodyLimit::max(25 * 1024 * 1024))
        .route("/embedding_pdf", post(embedding))
        .layer(DefaultBodyLimit::max(25 * 1024 * 1024))
        .route("/chat_collection", post(chat_collection))
        .with_state(state)
}
