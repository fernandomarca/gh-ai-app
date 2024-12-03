#![allow(dead_code)]

use crate::http::error_handling::AppError;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub async fn tags() -> Result<Json<ListResponse>, AppError> {
    let resp: ListResponse = reqwest::Client::new()
        .get("http://localhost:11434/api/tags")
        .send()
        .await?
        .json()
        .await?;
    Ok(Json(resp))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListResponse {
    models: Vec<Value>,
}

// {
// 	"models": [
// 		{
// 			"name": "llama3.2:latest",
// 			"model": "llama3.2:latest",
// 			"modified_at": "2024-12-02T14:35:31.068846269Z",
// 			"size": 2019393189,
// 			"digest": "a80c4f17acd55265feec403c7aef86be0c25983ab279d83f3bcd3abbcb5b8b72",
// 			"details": {
// 				"parent_model": "",
// 				"format": "gguf",
// 				"family": "llama",
// 				"families": [
// 					"llama"
// 				],
// 				"parameter_size": "3.2B",
// 				"quantization_level": "Q4_K_M"
// 			}
// 		}
// 	]
// }
