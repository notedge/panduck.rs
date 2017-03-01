use crate::{Result, ToNotedown};
use html5ever::{parse_document, rcdom::RcDom, tendril::TendrilSink};
use markdown::tokenize;
use notedown_ast::AST;

pub fn parse_markdown(input: &str) -> Result<AST> {
    Ok(tokenize(input).to_notedown())
}

pub fn parse_html(text: &str) -> Result<AST> {
    let dom = parse_document(RcDom::default(), Default::default()).from_utf8().read_from(&mut text.as_bytes())?;
    Ok(dom.to_notedown())
}
