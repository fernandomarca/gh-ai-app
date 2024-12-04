#![allow(dead_code)]

use crate::http::error_handling::AppError;
use axum::Json;
use axum::{
    body::{Body, Bytes},
    response::Response,
};
use futures::TryStreamExt;
use langchain_rust::embedding::OllamaEmbedder;
use langchain_rust::vectorstore::pgvector::StoreBuilder;
use langchain_rust::vectorstore::Retriever;
use langchain_rust::vectorstore::VecStoreOptions;
use langchain_rust::{
    chain::{Chain, ConversationalRetrieverChainBuilder},
    fmt_message, fmt_template,
    llm::client::{GenerationOptions, Ollama},
    memory::SimpleMemory,
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::Message,
    template_jinja2,
};
use serde::Deserialize;
use serde::Serialize;
pub async fn chat_collection(Json(payload): Json<AskRequest>) -> Result<Response, AppError> {
    let options = GenerationOptions::default()
        .temperature(payload.temperature.unwrap_or(0.0))
        .num_thread(payload.num_thread.unwrap_or(4));

    let llm = Ollama::default()
        .with_model(&payload.model)
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

    let ollama = OllamaEmbedder::default().with_model("mxbai-embed-large");

    let store = StoreBuilder::new()
        .embedder(ollama)
        .vstore_options(VecStoreOptions::default())
        .collection_name(&payload.collection)
        .connection_url("postgresql://postgres:123456@localhost:5432/postgres")
        .build()
        .await
        .map_err(|e| AppError(anyhow::Error::msg(format!("Error building store {}", e))))?;

    let chain = ConversationalRetrieverChainBuilder::new()
        .llm(llm)
        .return_source_documents(true)
        .rephrase_question(true)
        .memory(SimpleMemory::new().into())
        .retriever(Retriever::new(store, 10))
        .prompt(prompt)
        .build()
        .expect("Error building ConversationalChain");

    let input_variables = prompt_args! {
        "question" => payload.question,
    };

    let stream = chain.stream(input_variables).await;
    match stream {
        Ok(stream) => {
            let byte_stream = stream.map_ok(|data| Bytes::from(data.content.into_bytes()));
            let stream_body = Body::from_stream(byte_stream);
            Ok(Response::builder().status(200).body(stream_body).unwrap())
        }
        Err(e) => Err(AppError(anyhow::Error::msg(e.to_string()))),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AskRequest {
    model: String,
    question: String,
    collection: String,
    stream: Option<bool>,
    temperature: Option<f32>,
    num_thread: Option<u32>,
}
