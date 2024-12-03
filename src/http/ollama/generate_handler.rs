#![allow(dead_code)]

use crate::http::error_handling::AppError;
use axum::body::Body;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionsRequest {
    seed: Option<u32>,
    temperature: Option<f32>,
    num_thread: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateRequest {
    model: String,
    prompt: String,
    images: Option<Vec<String>>,
    options: Option<OptionsRequest>,
    stream: Option<bool>,
    format: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateResponse {
    model: String,
    created_at: String,
    response: Value,
    done: bool,
    done_reason: String,
    context: Vec<i32>,
    total_duration: i64,
    load_duration: i64,
    prompt_eval_count: i32,
    prompt_eval_duration: i64,
    eval_count: i32,
    eval_duration: i64,
}
pub async fn generate(
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    let resp: GenerateResponse = reqwest::Client::new()
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;
    Ok(Json(resp))
}

pub async fn generate_stream(Json(payload): Json<GenerateRequest>) -> Result<Response, AppError> {
    let reqwest_response = reqwest::Client::new()
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await?;
    let mut response_builder = Response::builder().status(reqwest_response.status());
    *response_builder.headers_mut().unwrap() = reqwest_response.headers().clone();
    Ok(response_builder
        .body(Body::from_stream(reqwest_response.bytes_stream()))
        .unwrap())
}
