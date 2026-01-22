use crate::chunk::AST;
use anyhow::Result;
// use docx_rust::document::Paragraph;
use docx_rust::{Docx, DocxFile};
use std::path::PathBuf;

pub struct DocxAST<'a> {
    ast: Docx<'a>,
}

impl AST for DocxAST<'_> {
    fn import(&mut self, filepath: PathBuf) -> Result<()> {
        let docx = DocxFile::from_file(filepath)?;
        self.ast = docx.parse()?;
        Ok(())
    }

    fn export(&self, filepath: PathBuf) -> Result<()> {
        todo!()
        // self.ast.write_file(filepath)?;
        // Ok(())
    }

    fn to_mipcs(&self) -> Vec<String> {
        todo!()
    }
    fn apply_mipcs(&mut self, mipcs: Vec<String>) -> Result<()> {
        todo!()
    }
}
