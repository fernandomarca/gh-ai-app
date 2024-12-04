#![allow(dead_code)]

use crate::http::error_handling::AppError;
use async_trait::async_trait;
use axum::extract::Multipart;
use futures::StreamExt;
use langchain_rust::{
    add_documents,
    document_loaders::{pdf_extract_loader::PdfExtractLoader, Loader},
    embedding::OllamaEmbedder,
    text_splitter::{TextSplitter, TextSplitterError},
    vectorstore::{pgvector::StoreBuilder, VecStoreOptions, VectorStore},
};
use std::collections::HashMap;
use std::sync::LazyLock;
use text_splitter::TextSplitter as Splitter;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub static VECTOR_DIMENSIONS: LazyLock<HashMap<&'static str, i32>> =
    LazyLock::new(|| HashMap::from([("mxbai-embed-large", 1024), ("llama3.2", 3072)]));
pub async fn embedding(mut multipart: Multipart) -> Result<(), AppError> {
    let current_dir = std::env::current_dir()?;
    let pdf_path = current_dir.join("src/upload/embedding.pdf");
    let mut file = File::create(&pdf_path).await?;

    let mut model = String::new();
    let mut collection_name = String::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field
            .name()
            .ok_or(AppError(anyhow::Error::msg("campo name não é suportado")))?
            .to_string();
        let data = field.bytes().await?;
        let text = String::from_utf8(data.to_vec()).unwrap_or_default();
        match name.as_str() {
            "model" => {
                if text.is_empty() {
                    return Err(AppError(anyhow::Error::msg("model não pode estar vazio")));
                }
                model = text;
            }
            "collection_name" => {
                if text.is_empty() {
                    return Err(AppError(anyhow::Error::msg(
                        "collection_name não pode estar vazio",
                    )));
                }
                collection_name = text;
            }
            "file" => {
                if data.is_empty() {
                    return Err(AppError(anyhow::Error::msg("file não pode estar vazio")));
                }
                file.write_all(&data).await?;
            }
            _ => {
                return Err(AppError(anyhow::Error::msg("campo name não é suportado")));
            }
        }
    }

    let ollama = OllamaEmbedder::default().with_model(&model);
    let loader = PdfExtractLoader::from_path(pdf_path)?;

    let splitter = MyTextSplitter {};

    let documents = loader
        .load_and_split(splitter)
        .await
        .unwrap()
        .map(|d| d.unwrap())
        .collect::<Vec<_>>()
        .await;

    let vector_dimensions = VECTOR_DIMENSIONS
        .get(model.as_str())
        .ok_or(AppError(anyhow::Error::msg("Modelo não suportado")))?
        .to_owned();

    let store = StoreBuilder::new()
        .embedder(ollama)
        .vstore_options(VecStoreOptions::default())
        .vector_dimensions(vector_dimensions)
        .pre_delete_collection(true)
        .collection_name(&collection_name)
        .connection_url("postgresql://postgres:123456@localhost:5432/postgres")
        .build()
        .await
        .map_err(|e| AppError(anyhow::Error::msg(format!("Error building store {}", e))))?;

    tokio::spawn(async move {
        if let Err(e) = add_documents!(store, &documents).await {
            println!("Error adding documents: {:?}", e);
        }
    });
    Ok(())
}

struct MyTextSplitter {}

#[async_trait]
impl TextSplitter for MyTextSplitter {
    async fn split_text(&self, text: &str) -> Result<Vec<String>, TextSplitterError> {
        let result = Splitter::new(2000)
            .chunks(text)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Ok(result)
    }
}
