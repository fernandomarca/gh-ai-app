#![allow(dead_code)]

use crate::config::environment::get_config_values;
use crate::http::error_handling::AppError;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub async fn ps() -> Result<Json<RunningResponse>, AppError> {
    let config = get_config_values();

    let resp: RunningResponse = reqwest::Client::new()
        .get(format!("{}/ps", config.get_ollama_server_url()))
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
