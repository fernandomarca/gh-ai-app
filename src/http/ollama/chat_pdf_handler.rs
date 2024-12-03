#![allow(dead_code)]

use crate::http::error_handling::AppError;
use async_trait::async_trait;
use axum::body::Body;
use axum::response::Response;
use axum::Json;
use futures::StreamExt;
use langchain_rust::chain::Chain;
use langchain_rust::chain::ConversationalRetrieverChainBuilder;
use langchain_rust::document_loaders::pdf_extract_loader::PdfExtractLoader;
use langchain_rust::document_loaders::Loader;
use langchain_rust::embedding::OllamaEmbedder;
use langchain_rust::fmt_message;
use langchain_rust::fmt_template;
use langchain_rust::llm::client::GenerationOptions;
use langchain_rust::llm::client::Ollama;
use langchain_rust::memory::SimpleMemory;
use langchain_rust::message_formatter;
use langchain_rust::prompt::HumanMessagePromptTemplate;
use langchain_rust::prompt_args;
use langchain_rust::schemas::Document;
use langchain_rust::schemas::Message;
use langchain_rust::schemas::Retriever;
use langchain_rust::template_jinja2;
use langchain_rust::text_splitter::TextSplitter;
use langchain_rust::text_splitter::TextSplitterError;
use langchain_rust::vectorstore::pgvector::StoreBuilder;
use langchain_rust::vectorstore::VecStoreOptions;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::error::Error;
use std::path::Path;
use text_splitter::TextSplitter as Splitter;

pub async fn chat_pdf(Json(payload): Json<AskRequest>) -> Result<(), AppError> {
    let options = GenerationOptions::default().temperature(0.0).num_thread(8);
    // let ollama = OllamaEmbedder::default().with_model(payload.model);
    // let loader = PdfExtractLoader::from_path("pops.pdf")?;

    // let splitter = MyTextSplitter {};

    // let documents = loader
    //     .load_and_split(splitter)
    //     .await
    //     .unwrap()
    //     .map(|d| d.unwrap())
    //     .collect::<Vec<_>>()
    //     .await;

    // let store = StoreBuilder::new()
    //     .embedder(ollama)
    //     .vstore_options(VecStoreOptions::default())
    //     .vector_dimensions(1024)
    //     .collection_name("pops")
    //     .connection_url("postgresql://postgres:123456@localhost:5432/postgres")
    //     .build()
    //     .await
    //     .unwrap();

    let llm = Ollama::default()
        .with_model(payload.model)
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

    let current_dir = std::env::current_dir().unwrap();
    let pdf_path = current_dir.join("src/pops.pdf");

    let pdf_retriever = PdfRetriever::new(pdf_path);

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
        "question" => payload.question
    };

    let mut stream = chain.stream(input_variables).await.unwrap();
    while let Some(result) = stream.next().await {
        match result {
            Ok(data) => data.to_stdout().unwrap(),
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
    Ok(())
    // let mut response_builder = Response::builder().status(reqwest_response.status());
    // *response_builder.headers_mut().unwrap() = reqwest_response.headers().clone();
    // Ok(response_builder
    //     .body(Body::from_stream(reqwest_response.bytes_stream()))
    //     .unwrap())
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
