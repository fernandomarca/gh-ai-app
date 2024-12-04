#![allow(dead_code)]

use crate::http::error_handling::AppError;
use axum::body::Body;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub async fn chat(Json(payload): Json<ChatRequest>) -> Result<Json<ChatResponse>, AppError> {
    let resp: ChatResponse = reqwest::Client::new()
        .post("http://localhost:11434/api/chat")
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;
    Ok(Json(resp))
}

pub async fn chat_stream(Json(payload): Json<ChatRequest>) -> Result<Response, AppError> {
    let reqwest_response = reqwest::Client::new()
        .post("http://localhost:11434/api/chat")
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
pub struct OptionsRequest {
    seed: Option<u32>,
    temperature: Option<f32>,
    num_thread: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageRequest {
    role: String,
    content: String,
    images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatRequest {
    model: String,
    messages: Vec<MessageRequest>,
    options: Option<OptionsRequest>,
    stream: Option<bool>,
    format: Option<String>,
    keep_alive: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    model: String,
    created_at: String,
    message: Value,
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

// {
// 	"model": "llama3.2",
// 	"messages": [
// 		{
// 			"role": "user",
// 			"content": "why is the sky blue?"
// 		},
// 		{
// 			"role": "assistant",
// 			"content": "due to rayleigh scattering."
// 		},
// 		{
// 			"role": "user",
// 			"content": "how is that different than mie scattering?"
// 		}
// 	],
// "options": {
// 	"seed": 101,
// 	"temperature": 0
// },
// "stream": false
// }
