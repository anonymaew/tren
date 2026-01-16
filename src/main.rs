use anyhow::{Result, anyhow};
// use aws_sdk_s3::{Client as S3Client, types::ByteStream};
// use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
// use sqlx::{PgPool, postgres::PgPoolOptions};
// use std::{env, fs, sync::Arc};
// use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    response: String,
}
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    #[serde(rename = "system")]
    system_prompt: Option<String>,
    prompt: String,
}
#[derive(Serialize, Deserialize)]
struct OpenAIOptions {
    temperature: f32,
}
#[derive(Serialize, Deserialize)]
enum JobStatus {
    Waiting,
    Running,
    Success,
    Error(String),
}
#[derive(Serialize, Deserialize)]
struct Job {
    id: Uuid,
    source_lang: String,
    target_lang: String,

    model: String,
    system_prompt: Option<String>,
    user_prompt: String,
    // chunking: ChunkType,
    options: OpenAIOptions,

    status: String,
    input_file: String,
    output_file: String,
}

use tren::chunk::{markdown_to_mipcs, mipcs_to_markdown};

fn main() -> Result<()> {
    let filename = "./assets/chinese-text.md";
    let file = std::path::PathBuf::from(filename);
    let content = std::fs::read_to_string(file)?;

    let micps = markdown_to_mipcs(&content)?;

    let modified_micps = micps
        .iter()
        .map(|micp| micp.replace("。", ""))
        .collect::<Vec<_>>();
    // let modified_micps = micps.clone();

    let modified_markdown = mipcs_to_markdown(&content, modified_micps)?;

    println!("{modified_markdown}");

    Ok(())
}
