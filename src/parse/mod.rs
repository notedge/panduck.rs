use crate::{Result, ToNotedown};
use html_parser::Dom;
use markdown::tokenize;
use notedown_ast::AST;

pub fn parse_markdown(input: &str) -> Result<AST> {
    Ok(tokenize(input).to_notedown())
}

pub fn parse_html(text: &str) -> Result<AST> {
    let dom = Dom::parse(text)?;
    Ok(dom.to_notedown())
}
