use clap::{Parser, Subcommand};
use dotenv;
use std::ffi::{OsStr, OsString};
use std::net::IpAddr;
use std::path::{Path, PathBuf};

#[derive(Parser)]
pub struct CLI {
    #[command(subcommand)]
    pub mode: CLIMode,
}

#[derive(Subcommand)]
pub enum CLIMode {
    /// Translate a single file on CLI
    Run(JobCLIArgs),
    /// Start a web server; can submit translation jobs via web UI
    Web(WebCLIArgs),
}

#[derive(Parser, Debug)]
pub struct JobCLIArgs {
    /// Source language.
    #[arg(long)]
    src: String,

    /// Target language.
    #[arg(long)]
    tar: String,

    /// Input file
    #[arg(short, long)]
    input: PathBuf,

    /// Intermediate sheet for inspecting/editing translation results. Must be csv.
    #[arg(long)]
    inter_sheet: Option<PathBuf>,

    /// Output file. [default: <INPUT>-translated]
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// LLM model name; should be a Huggingface repo name.
    #[arg(long, default_value = "openai/gpt-oss-20b")]
    model: String,

    /// System prompt for LLM.
    #[arg(long)]
    system: Option<String>,

    /// User prompt for LLM.
    #[arg(long)]
    user: Option<String>,

    /// Maximum parallel request to LLM.
    #[arg(short = 'j', long, default_value = "1")]
    parallel: usize,
}

#[derive(Parser, Debug)]
pub struct WebCLIArgs {
    /// Host address
    #[arg(long, default_value = "0.0.0.0")]
    pub host: IpAddr,

    /// Port number
    #[arg(long, default_value = "8080")]
    pub port: u16,

    /// Data directory for storing stuff
    #[arg(long, default_value = "./data")]
    pub data: PathBuf,

    /// Whether to enable authentication; which also allows multiple users
    #[arg(long)]
    pub auth: bool,

    /// Maximum parallel job processing at one time
    #[arg(short = 'j', long, default_value = "1")]
    pub parallel: usize,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub src: String,
    pub tar: String,
    pub input: PathBuf,
    pub inter_sheet: PathBuf,
    pub output: PathBuf,
    pub llm: LLM,
    pub system: String,
    pub user: String,
    pub parallel: usize,
}

#[derive(Debug, Clone)]
pub struct LLM {
    pub url: String,
    pub api_key: Option<String>,
    pub model: String,
}

fn suffix_fallback(
    src: &Option<PathBuf>,
    template: &PathBuf,
    suffix: String,
    overwritten_extension: Option<&OsStr>,
) -> PathBuf {
    src.clone().unwrap_or_else(|| {
        let input = template.clone();
        let parent = input.parent().unwrap_or(Path::new(""));
        let file_name = input.file_name().unwrap_or(OsStr::new(""));
        let stem = Path::new(file_name).file_stem().unwrap_or(OsStr::new(""));
        let ext = overwritten_extension.or(Path::new(file_name).extension());
        let mut new_name = OsString::from(stem);
        new_name.push(suffix);
        if let Some(ext) = ext {
            new_name.push(".");
            new_name.push(ext);
        } else if let Some(ext) = overwritten_extension {
            new_name.push(".");
            new_name.push(ext);
        }
        parent.join(new_name)
    })
}

pub fn transform_job_cli(job_cli: JobCLIArgs) -> Job {
    dotenv::dotenv().ok();

    let system_prompt = "You are an expert translator. Please translate {{ source_language }} into {{ target_language }}. The user will submit sentences or paragraphs with some contexts; please only translate the intended text into {{ target_language }}.

- If there are symbols {{ special_tokens | join(\" , \") }}, keep the symbol intact on the result text in the correct position.
- Do not give any alternative translation or including any previous context, notes or discussion.".to_string();
    let user_prompt = "
{%- set previous_chunks = previous_chunks[-8:] -%}
{%- if previous_chunks -%}
Given the previous context:

{{ previous_chunks | join(\"\\n\\n\") }}

Only translate the following text:

{% endif -%}
{{ source_text }}"
        .to_string();

    return Job {
        inter_sheet: suffix_fallback(
            &job_cli.inter_sheet,
            &job_cli.input,
            "-inter".to_string(),
            Some(OsStr::new("csv")),
        ),
        output: suffix_fallback(
            &job_cli.output,
            &job_cli.input,
            "-translated".to_string(),
            None,
        ),
        system: job_cli.system.clone().unwrap_or(system_prompt),
        user: job_cli.user.clone().unwrap_or(user_prompt),
        src: job_cli.src,
        tar: job_cli.tar,
        input: job_cli.input,
        llm: LLM {
            url: std::env::var_os("OPENAI_API_BASE")
                .unwrap_or("https://api.openai.com/v1".into())
                .into_string()
                .unwrap(),
            api_key: std::env::var_os("OPENAI_API_KEY").map(|s| s.into_string().unwrap()),
            model: job_cli.model,
        },
        parallel: job_cli.parallel,
    };
}
