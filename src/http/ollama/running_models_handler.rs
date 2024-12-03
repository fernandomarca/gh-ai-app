#![allow(dead_code)]

use crate::http::error_handling::AppError;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub async fn ps() -> Result<Json<RunningResponse>, AppError> {
    let resp: RunningResponse = reqwest::Client::new()
        .get("http://localhost:11434/api/ps")
        .send()
        .await?
        .json()
        .await?;
    Ok(Json(resp))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RunningResponse {
    models: Vec<Value>,
}
