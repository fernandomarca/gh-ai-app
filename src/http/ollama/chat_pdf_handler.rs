#![allow(dead_code)]

use crate::http::error_handling::AppError;
use async_trait::async_trait;
use axum::body::Body;
use axum::body::Bytes;
use axum::extract::Multipart;
use axum::response::Response;
use futures::StreamExt;
use futures::TryStreamExt;
use langchain_rust::{
    chain::{Chain, ConversationalRetrieverChainBuilder},
    document_loaders::{pdf_extract_loader::PdfExtractLoader, Loader},
    fmt_message, fmt_template,
    llm::client::{GenerationOptions, Ollama},
    memory::SimpleMemory,
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::{Document, Message, Retriever},
    template_jinja2,
    text_splitter::{TextSplitter, TextSplitterError},
};
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use text_splitter::TextSplitter as Splitter;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
pub async fn chat_pdf(mut multipart: Multipart) -> Result<Response, AppError> {
    let current_dir = std::env::current_dir()?;
    let pdf_path = current_dir.join("src/upload/file.pdf");
    let mut file = File::create(&pdf_path).await?;

    let mut model_value = String::new();
    let mut question_value = String::new();

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
                model_value = text;
            }
            "question" => {
                if text.is_empty() {
                    return Err(AppError(anyhow::Error::msg(
                        "question não pode estar vazio",
                    )));
                }
                question_value = text;
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
    let options = GenerationOptions::default().temperature(0.0).num_thread(8);

    let llm = Ollama::default()
        .with_model(model_value)
        .with_options(options);

    let prompt= message_formatter![
                  fmt_message!(Message::new_system_message("Você é um assistente útil")),
                  fmt_template!(HumanMessagePromptTemplate::new(
                  template_jinja2!(
                      "Use as seguintes partes do contexto para responder à pergunta no final. 
                      Se você não sabe a resposta, apenas diga que não sabe, não tente inventar uma resposta.
                      {{context}}
                  
                      Pergunta: {{question}}
                      Resposta útil:",
                      "context",
                      "question"
                  )))];

    let pdf_retriever = PdfRetriever::new(pdf_path.clone());

    let chain = ConversationalRetrieverChainBuilder::new()
        .llm(llm)
        .return_source_documents(true)
        .rephrase_question(true)
        .memory(SimpleMemory::new().into())
        .retriever(pdf_retriever)
        .prompt(prompt)
        .build()
        .expect("Error building ConversationalChain");

    let input_variables = prompt_args! {
        "question" => question_value,
    };

    let stream = chain.stream(input_variables).await;
    match stream {
        Ok(stream) => {
            let byte_stream = stream.map_ok(|data| Bytes::from(data.content.into_bytes()));
            let stream_body = Body::from_stream(byte_stream);
            fs::remove_file(&pdf_path).await?;
            Ok(Response::builder().status(200).body(stream_body).unwrap())
        }
        Err(e) => {
            fs::remove_file(&pdf_path).await?;
            Err(AppError(anyhow::Error::msg(e.to_string())))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AskRequest {
    model: String,
    question: String,
    stream: Option<bool>,
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

pub struct PdfRetriever {
    pdf_path: std::path::PathBuf,
}

impl PdfRetriever {
    pub fn new(pdf_path: std::path::PathBuf) -> Self {
        PdfRetriever { pdf_path }
    }
}

#[async_trait]
impl Retriever for PdfRetriever {
    async fn get_relevant_documents(
        &self,
        _question: &str,
    ) -> Result<Vec<Document>, Box<dyn Error>> {
        let loader = PdfExtractLoader::from_path(&self.pdf_path)?;
        let splitter = MyTextSplitter {};
        let documents = loader
            .load_and_split(splitter)
            .await
            .unwrap()
            .map(|d| d.unwrap())
            .collect::<Vec<_>>()
            .await;
        Ok(documents)
    }
}

// async fn embedding(path: &str, store: Store) {
//     let ollama = OllamaEmbedder::default().with_model("mxbai-embed-large");
//     let loader = PdfExtractLoader::from_path(path).expect("Failed to create PdfExtractLoader");

//     let splitter = MyTextSplitter {};

//     let documents = loader
//         .load_and_split(splitter)
//         .await
//         .unwrap()
//         .map(|d| d.unwrap())
//         .collect::<Vec<_>>()
//         .await;

//     for doc in &documents {
//         let _ = add_documents!(store, &documents).await.map_err(|e| {
//             println!("Error adding documents: {:?}", e);
//         });
//     }
// }
