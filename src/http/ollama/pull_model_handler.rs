#![allow(dead_code)]

use crate::http::error_handling::AppError;
use axum::body::Body;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

pub async fn pull_stream(Json(payload): Json<PullRequest>) -> Result<Response, AppError> {
    let reqwest_response = reqwest::Client::new()
        .post("http://localhost:11434/api/pull")
        .json(&payload)
        .send()
        .await?;
    let mut response_builder = Response::builder().status(reqwest_response.status());
    *response_builder.headers_mut().unwrap() = reqwest_response.headers().clone();
    Ok(response_builder
        .body(Body::from_stream(reqwest_response.bytes_stream()))
        .unwrap())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PullRequest {
    model: String,
    stream: Option<bool>,
}
