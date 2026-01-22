mod chunk;
use crate::chunk::{AST, pandoc_ast::PandocAST};
use anyhow::Result;
mod cli;
use cli::get_args;
mod translate;
use translate::task;

#[tokio::main]
async fn main() -> Result<()> {
    let args = get_args();

    let mut ast = PandocAST::default();
    ast.import(&args.input)?;

    let micps = ast.to_mipcs();
    let modified_mipcs = task(micps, &args).await?;
    ast.apply_mipcs(modified_mipcs.clone())?;

    ast.export(&args.output)?;

    Ok(())
}
