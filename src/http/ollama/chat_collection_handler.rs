#![allow(dead_code)]

use crate::config::environment::ConfigModule;
use crate::http::error_handling::AppError;
use axum::extract::State;
use axum::Json;
use axum::{
    body::{Body, Bytes},
    response::Response,
};
use futures::TryStreamExt;
use langchain_rust::{
    chain::{Chain, ConversationalRetrieverChainBuilder},
    embedding::OllamaEmbedder,
    fmt_message, fmt_template,
    llm::client::{GenerationOptions, Ollama, OllamaClient},
    memory::SimpleMemory,
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::Message,
    template_jinja2,
    vectorstore::{pgvector::StoreBuilder, Retriever, VecStoreOptions},
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
pub async fn chat_collection(
    State(state): State<ConfigModule>,
    Json(payload): Json<AskRequest>,
) -> Result<Response, AppError> {
    let options = GenerationOptions::default()
        .temperature(payload.temperature.unwrap_or(0.0))
        .num_thread(payload.num_thread.unwrap_or(4));

    let llm_client = Arc::new(OllamaClient::new(
        format!("http://{}", &state.ollama_server_host),
        state.ollama_server_port.parse::<u16>()?,
    ));
    let llm = Ollama::new(llm_client.clone(), &payload.model, Some(options));
    let embedder = OllamaEmbedder::new(llm_client.clone(), "mxbai-embed-large", None);

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

    let store = StoreBuilder::new()
        .embedder(embedder)
        .vstore_options(VecStoreOptions::default())
        .collection_name(&payload.collection)
        .connection_url(&state.get_database_url())
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
            // let byte_stream = stream.map_ok(|data| Bytes::from(data.content.into_bytes()));
            let byte_stream = stream
                .map_ok(|data| Bytes::from(format!("{}\n", json!({"response":data.content}))));
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
