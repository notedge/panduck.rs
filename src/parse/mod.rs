use notedown_ast::AST;
use markdown::tokenize;
use crate::ToNotedown;
use html5ever::parse_document;
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;
use crate::Result;


pub fn parse_markdown(input: &str) -> Result<AST> {
    Ok(tokenize(input).to_notedown())
}

pub fn parse_html(text: &str) -> Result<AST> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut text.as_bytes())?;
    Ok(dom.to_notedown())
}
