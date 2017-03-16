use crate::{convert::jupyter_from_json, error::Error::UnsupportedFormat, Result, ToNotedown};
use html_parser::Dom;
use markdown::tokenize;
use notedown_parser::{ParserConfig};
use serde_json::Value;
use std::{fs::read_to_string, path::Path};
use crate::convert::AST;

pub fn parse_by_ext(file: impl AsRef<Path>) -> Result<AST> {
    let ext = file.as_ref().extension().unwrap_or_default().to_str().unwrap_or_default();
    let input = &read_to_string(file.as_ref())?;
    match ext {
        "note" | "notedown" => parse_notedown(input),
        "md" | "markdown" => parse_markdown(input),
        "html" => parse_html(input),
        "ipynb" => parse_jupyter(input),
        _ => Err(UnsupportedFormat(String::from(ext))),
    }
}

pub fn parse_notedown(input: &str) -> Result<AST> {
    Ok(AST::from(ParserConfig::default().parse(input)?))
}

pub fn parse_markdown(input: &str) -> Result<AST> {
    Ok(tokenize(input).to_notedown())
}

pub fn parse_html(text: &str) -> Result<AST> {
    Ok(Dom::parse(text)?.to_notedown())
}

pub fn parse_jupyter(text: &str) -> Result<AST> {
    let v: Value = serde_json::from_str(text)?;
    Ok(jupyter_from_json(&v)?)
}
