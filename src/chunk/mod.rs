use anyhow::Result;
// use pandoc_types::definition::{Inline, *};
use std::path::PathBuf;

pub const TOK_SEP: char = '𐑙';

pub mod pandoc_ast;

pub trait AST {
    fn import(&mut self, filepath: PathBuf) -> Result<()>;
    fn to_mipcs(&self) -> Vec<String>;
    fn apply_mipcs(&mut self, mipcs: Vec<String>) -> Result<()>;
    fn export(&self, filepath: PathBuf) -> Result<()>;
}
