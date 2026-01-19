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

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use tren::{
    chunk::{AST, pandoc_ast::PandocAST},
    translate::task,
};

pub fn add_suffix(src: &PathBuf, suffix: String) -> PathBuf {
    let parent = src.parent().unwrap_or(Path::new(""));
    let file_name = src.file_name().unwrap_or(OsStr::new(""));
    let stem = Path::new(file_name).file_stem().unwrap_or(OsStr::new(""));
    let ext = Path::new(file_name).extension();
    let mut new_name = OsString::from(stem);
    new_name.push(suffix);
    if let Some(ext) = ext {
        new_name.push(".");
        new_name.push(ext);
    }
    parent.join(new_name)
}

#[tokio::main]
async fn main() -> Result<()> {
    let filename = Path::new("./assets/chinese-text.md").to_path_buf();
    let tar_filename = add_suffix(&filename, "-translate".to_string());

    let mut ast = PandocAST::default();
    ast.import(filename)?;

    let micps = ast.to_mipcs();
    let modified_mipcs = task(micps).await?;
    ast.apply_mipcs(modified_mipcs.clone())?;

    ast.export(tar_filename)?;

    Ok(())
}
