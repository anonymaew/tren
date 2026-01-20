use crate::chunk::{AST, TOK_SEP};
use anyhow::{Result, anyhow};
use pandoc_types::definition::{Inline, *};
use serde_json::json;
use std::path::PathBuf;
use std::vec::IntoIter;

fn collect_ins(bs: &Vec<Block>) -> Vec<String> {
    bs.iter()
        .filter_map(|b| match b {
            Block::Plain(ins) | Block::Para(ins) | Block::Header(_, _, ins) => {
                fn inlines_to_strings(ins: &Vec<Inline>) -> Vec<String> {
                    ins.iter()
                        .flat_map(|inline| match inline {
                            Inline::Str(str) | Inline::RawInline(_, str) => vec![str.clone()],
                            Inline::Code(_, _)
                            | Inline::Space
                            | Inline::SoftBreak
                            | Inline::LineBreak
                            | Inline::Math(_, _)
                            | Inline::Note(_) => vec![],
                            Inline::Cite(_, _ins) => todo!(),
                            Inline::Span(_, ins)
                            | Inline::Link(_, ins, _)
                            | Inline::Image(_, ins, _)
                            | Inline::Emph(ins)
                            | Inline::Strong(ins)
                            | Inline::Underline(ins)
                            | Inline::Strikeout(ins)
                            | Inline::Superscript(ins)
                            | Inline::Subscript(ins)
                            | Inline::SmallCaps(ins)
                            | Inline::Quoted(_, ins) => inlines_to_strings(ins),
                        })
                        .collect()
                }

                Some(vec![inlines_to_strings(ins).join(&TOK_SEP.to_string())])
            }
            Block::LineBlock(_inss) => todo!(),
            Block::CodeBlock(_, _) => None,
            Block::RawBlock(_format, _text) => None,
            Block::BlockQuote(bs) | Block::Div(_, bs) => Some(collect_ins(bs)),
            Block::OrderedList(_, _bss) => todo!(),
            Block::BulletList(_bss) => todo!(),
            Block::DefinitionList(_terms) => todo!(),
            Block::HorizontalRule | Block::Null => None,
            Block::Table(_) => todo!(),
            Block::Figure(_, _cap, _bs) => todo!(),
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn apply_mipc_to_blocks(bs_ref: &mut Vec<Block>, mipc_iter: &mut IntoIter<String>) {
    bs_ref.iter_mut().for_each(|b_ref| match b_ref {
        Block::Plain(ins) | Block::Para(ins) | Block::Header(_, _, ins) => {
            let strings = mipc_iter
                .next()
                .expect("mismatch")
                .split(TOK_SEP)
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            fn strings_to_inlines(inlines: &mut Vec<Inline>, str_iter: &mut IntoIter<String>) {
                inlines.iter_mut().for_each(|inline| match inline {
                    Inline::Str(i) | Inline::RawInline(_, i) => {
                        *i = str_iter.next().expect("mismatch");
                    }
                    Inline::Code(_, _)
                    | Inline::Space
                    | Inline::SoftBreak
                    | Inline::LineBreak
                    | Inline::Math(_, _)
                    | Inline::Note(_) => (),
                    Inline::Cite(_, _ins) => todo!(),
                    Inline::Span(_, ins)
                    | Inline::Link(_, ins, _)
                    | Inline::Image(_, ins, _)
                    | Inline::Emph(ins)
                    | Inline::Strong(ins)
                    | Inline::Underline(ins)
                    | Inline::Strikeout(ins)
                    | Inline::Superscript(ins)
                    | Inline::Subscript(ins)
                    | Inline::SmallCaps(ins)
                    | Inline::Quoted(_, ins) => strings_to_inlines(ins, str_iter),
                });
            }

            strings_to_inlines(ins, &mut strings.into_iter());
        }
        Block::LineBlock(_inss) => todo!(),
        Block::CodeBlock(_, _) => (),
        Block::RawBlock(_format, _text) => (),
        Block::BlockQuote(bs) | Block::Div(_, bs) => apply_mipc_to_blocks(bs, mipc_iter),
        Block::OrderedList(_, _bss) => todo!(),
        Block::BulletList(_bss) => todo!(),
        Block::DefinitionList(_terms) => todo!(),
        Block::HorizontalRule | Block::Null => (),
        Block::Table(_) => todo!(),
        Block::Figure(_, _cap, _bs) => todo!(),
    })
}

fn clean_space(bs: &mut Vec<Block>) {
    bs.iter_mut().for_each(|b| match b {
        Block::Plain(ins) | Block::Para(ins) | Block::Header(_, _, ins) => {
            fn clean_space_inlines(ins: &mut Vec<Inline>) {
                ins.iter_mut().for_each(|inline| {
                    match inline {
                        Inline::Space => *inline = Inline::Str(" ".to_string()),
                        Inline::Link(_, ins, _)
                        | Inline::Image(_, ins, _)
                        | Inline::Emph(ins)
                        | Inline::Strong(ins)
                        | Inline::Underline(ins)
                        | Inline::Strikeout(ins)
                        | Inline::Superscript(ins)
                        | Inline::Subscript(ins)
                        | Inline::SmallCaps(ins)
                        | Inline::Quoted(_, ins) => clean_space_inlines(ins),
                        _ => (),
                    };
                });
                *ins = ins
                    .iter()
                    .fold(vec![], |accs: Vec<Inline>, inline: &Inline| {
                        match (accs.last(), inline) {
                            (None, _) => vec![inline.clone()],
                            (Some(Inline::Str(str1)), Inline::Str(str2)) => {
                                let mut new_accs = accs.clone();
                                *new_accs.last_mut().unwrap() =
                                    Inline::Str(format!("{str1}{str2}"));
                                new_accs
                            }
                            _ => accs.to_vec(),
                        }
                    })
            }
            clean_space_inlines(ins);
        }
        Block::LineBlock(_inss) => todo!(),
        Block::CodeBlock(_, _) => (),
        Block::RawBlock(_format, _text) => (),
        Block::BlockQuote(bs) | Block::Div(_, bs) => clean_space(bs),
        Block::OrderedList(_, _bss) => todo!(),
        Block::BulletList(_bss) => todo!(),
        Block::DefinitionList(_terms) => todo!(),
        Block::HorizontalRule | Block::Null => (),
        Block::Table(_) => todo!(),
        Block::Figure(_, _cap, _bs) => todo!(),
    });
}

#[derive(Clone, Default)]
pub struct PandocAST {
    ast: Vec<Block>,
}

impl AST for PandocAST {
    fn import(&mut self, filepath: PathBuf) -> Result<()> {
        let mut pandoc = pandoc::new();
        pandoc.set_input(pandoc::InputKind::Files(vec![filepath]));
        pandoc.set_output_format(pandoc::OutputFormat::Json, vec![]);
        pandoc.set_output(pandoc::OutputKind::Pipe);

        let result = pandoc.execute()?;
        let result_buf = match result {
            pandoc::PandocOutput::ToBuffer(buf) => Ok(buf),
            _ => Err(anyhow!("buf not found?")),
        }?;
        let ast = serde_json::from_str::<Pandoc>(&result_buf)?;
        self.ast = ast.blocks;

        clean_space(&mut self.ast);

        Ok(())
    }

    fn export(&self, filepath: PathBuf) -> Result<()> {
        let mut pandoc = pandoc::new();
        pandoc.set_input(pandoc::InputKind::Pipe(
            json!({
                "pandoc-api-version": [1,23,1],
                "meta": {},
                "blocks": &self.ast
            })
            .to_string(),
        ));
        pandoc.set_input_format(pandoc::InputFormat::Json, vec![]);
        pandoc.set_output_format(pandoc::OutputFormat::Markdown, vec![]);
        pandoc.set_output(pandoc::OutputKind::File(filepath));

        pandoc.execute()?;
        Ok(())
    }

    fn to_mipcs(&self) -> Vec<String> {
        collect_ins(&self.ast)
    }

    fn apply_mipcs(&mut self, mipcs: Vec<String>) -> Result<()> {
        let mut mipc_iter = mipcs.into_iter();

        apply_mipc_to_blocks(&mut self.ast, &mut mipc_iter);
        Ok(())
    }
}
