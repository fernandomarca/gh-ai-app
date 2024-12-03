#![allow(dead_code)]

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
    let reqwest_response = reqwest::Client::new()
        .delete("http://localhost:11434/api/delete")
        .json(&payload)
        .send()
        .await?;
    let mut response_builder = Response::builder().status(reqwest_response.status());
    *response_builder.headers_mut().unwrap() = reqwest_response.headers().clone();
    Ok(response_builder
        .body(Body::from_stream(reqwest_response.bytes_stream()))
        .unwrap())
}
