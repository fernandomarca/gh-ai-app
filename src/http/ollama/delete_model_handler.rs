#![allow(dead_code)]

use crate::config::environment::get_config_values;
use crate::http::error_handling::AppError;
use axum::body::Body;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteRequest {
    model: String,
}

pub async fn delete_stream(Json(payload): Json<DeleteRequest>) -> Result<Response, AppError> {
    let config = get_config_values();

    let reqwest_response = reqwest::Client::new()
        .delete(format!("{}/delete", config.get_ollama_server_url()))
        .json(&payload)
        .send()
        .await?;
    let mut response_builder = Response::builder().status(reqwest_response.status());
    *response_builder.headers_mut().unwrap() = reqwest_response.headers().clone();
    Ok(response_builder
        .body(Body::from_stream(reqwest_response.bytes_stream()))
        .unwrap())
}
