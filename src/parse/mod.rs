use crate::{convert::jupyter_from_json, Result, ToNotedown};
use html_parser::Dom;
use markdown::tokenize;
use notedown_parser::{AST, ParserConfig};
use serde_json::Value;

pub fn parse_notedown(input: &str)-> Result<AST> {
    let cfg = ParserConfig::default();
    Ok(cfg.parse(input)?)
}

pub fn parse_markdown(input: &str) -> Result<AST> {
    Ok(tokenize(input).to_notedown())
}

pub fn parse_html(text: &str) -> Result<AST> {
    let dom = Dom::parse(text)?;
    Ok(dom.to_notedown())
}

pub fn parse_jupyter(text: &str) -> Result<AST> {
    let v: Value = serde_json::from_str(text)?;
    Ok(jupyter_from_json(&v)?)
}
