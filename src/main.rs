mod chunk;
mod cli;
mod translate;
use crate::cli::{CLI, CLIMode, transform_job_cli};
use crate::translate::process_job;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let cli_val = CLI::parse();

    match cli_val.mode {
        CLIMode::Run(job_cli) => {
            let job = transform_job_cli(job_cli);
            process_job(&job).await?;
        }
        CLIMode::Web(web_cli) => {
            todo!()
        }
    }

    Ok(())
}
