use clap::Parser;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(about)]
struct ArgsCLI {
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
}

pub struct Args {
    pub src: String,
    pub tar: String,
    pub input: PathBuf,
    pub inter_sheet: PathBuf,
    pub output: PathBuf,
    pub model: String,
    pub system: String,
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

use minijinja::{context, render};

pub fn get_args() -> Args {
    let args_cli = ArgsCLI::parse();

    let special_tokens = vec!["𐑣"];
    let system_prompt = "You are an expert translator. Please translate {{ source_language }} into {{ target_language }}. The user will submit sentences or paragraphs; please only translate that portion of text into {{ target_language }}.

- If there are symbols {{ special_tokens | join(\" , \") }}, keep the symbol intact on the result text in the correct position.
- Do not give any alternative translation or including any notes or discussion.".to_string();

    // all context is here
    let ctx = context! {
        source_language => args_cli.src,
        target_language => args_cli.tar,
        special_tokens => special_tokens
    };

    return Args {
        inter_sheet: suffix_fallback(
            &args_cli.inter_sheet,
            &args_cli.input,
            "-inter".to_string(),
            Some(OsStr::new("csv")),
        ),
        output: suffix_fallback(
            &args_cli.output,
            &args_cli.input,
            "-translated".to_string(),
            None,
        ),
        system: render!(
            // use arg value or fallback to the hardcode one
            &args_cli.system.clone().unwrap_or(system_prompt),
            ctx
        ),
        src: args_cli.src,
        tar: args_cli.tar,
        input: args_cli.input,
        model: args_cli.model,
    };
}
