use anyhow::{Result, anyhow};
use pandoc_types::definition::{Inline, *};
use std::vec::IntoIter;

pub const TOK_SEP: char = '𐑙';

fn inlines_to_mipc(inlines: Vec<Inline>) -> String {
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

    inlines_to_strings(&inlines).join(&TOK_SEP.to_string())
}

fn mipc_to_inlines(ins_src: &mut Vec<Inline>, mipc: String) {
    let strings = mipc
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

    strings_to_inlines(ins_src, &mut strings.into_iter());
}

fn collect_ins(b: &Block) -> Vec<Vec<Inline>> {
    match b {
        Block::Plain(ins) | Block::Para(ins) | Block::Header(_, _, ins) => vec![ins.clone()],
        Block::LineBlock(_inss) => todo!(),
        Block::CodeBlock(_, _) => vec![],
        Block::RawBlock(_format, _text) => vec![],
        Block::BlockQuote(bs) | Block::Div(_, bs) => {
            bs.iter().flat_map(|b| collect_ins(&b)).collect()
        }
        Block::OrderedList(_, _bss) => todo!(),
        Block::BulletList(_bss) => todo!(),
        Block::DefinitionList(_terms) => todo!(),
        Block::HorizontalRule | Block::Null => vec![],
        Block::Table(_) => todo!(),
        Block::Figure(_, _cap, _bs) => todo!(),
    }
}

fn apply_mipc_to_block(b_ref: &mut Block, mipc_iter: &mut IntoIter<String>) {
    match b_ref {
        Block::Plain(ins) | Block::Para(ins) | Block::Header(_, _, ins) => {
            mipc_to_inlines(ins, mipc_iter.next().expect("mismatch"));
        }
        Block::LineBlock(_inss) => todo!(),
        Block::CodeBlock(_, _) => (),
        Block::RawBlock(_format, _text) => (),
        Block::BlockQuote(bs) | Block::Div(_, bs) => bs
            .iter_mut()
            .for_each(|b| apply_mipc_to_block(b, mipc_iter)),
        Block::OrderedList(_, _bss) => todo!(),
        Block::BulletList(_bss) => todo!(),
        Block::DefinitionList(_terms) => todo!(),
        Block::HorizontalRule | Block::Null => (),
        Block::Table(_) => todo!(),
        Block::Figure(_, _cap, _bs) => todo!(),
    };
}

fn markdown_to_ast(markdown: &String) -> Result<Vec<Block>> {
    let mut pandoc = pandoc::new();
    pandoc.set_input(pandoc::InputKind::Pipe(markdown.to_string()));
    pandoc.set_input_format(pandoc::InputFormat::Markdown, vec![]);
    pandoc.set_output_format(pandoc::OutputFormat::Json, vec![]);
    pandoc.set_output(pandoc::OutputKind::Pipe);

    let result = pandoc.execute()?;
    let result_buf = match result {
        pandoc::PandocOutput::ToBuffer(buf) => Ok(buf),
        _ => Err(anyhow!("buf not found?")),
    }?;
    let ast = serde_json::from_str::<Pandoc>(&result_buf)?;
    return Ok(ast.blocks);
}

fn ast_to_markdown(blocks: Vec<Block>) -> Result<String> {
    let pandoc_obj = Pandoc {
        blocks,
        meta: std::collections::HashMap::new(),
    };
    let mut pandoc = pandoc::new();
    pandoc.set_input(pandoc::InputKind::Pipe(serde_json::to_string(&pandoc_obj)?));
    pandoc.set_input_format(pandoc::InputFormat::Json, vec![]);
    pandoc.set_output_format(pandoc::OutputFormat::Markdown, vec![]);
    pandoc.set_output(pandoc::OutputKind::Pipe);

    let result = pandoc.execute()?;
    let result_buf = match result {
        pandoc::PandocOutput::ToBuffer(buf) => Ok(buf),
        _ => Err(anyhow!("buf not found?")),
    }?;
    return Ok(result_buf);
}

pub fn markdown_to_mipcs(markdown: &String) -> Result<Vec<String>> {
    Ok(markdown_to_ast(&markdown)?
        .iter()
        .flat_map(collect_ins)
        .map(inlines_to_mipc)
        .collect::<Vec<_>>())
}

pub fn mipcs_to_markdown(markdown_ref: &String, mipcs: Vec<String>) -> Result<String> {
    let mut ast_ref = markdown_to_ast(&markdown_ref)?;
    let mut mipc_iter = mipcs.into_iter();

    ast_ref.iter_mut().for_each(|block| {
        apply_mipc_to_block(block, &mut mipc_iter);
    });

    ast_to_markdown(ast_ref)
}
