use std::cell::RefCell;
use comrak::{Arena, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, parse_document};
use comrak::arena_tree::Node;
use comrak::nodes::{Ast, AstNode, NodeMath, NodeValue};
use wasi_notedown::exports::notedown::core::syntax_tree::{MathContent, MathDisplay, MathSpan, NotedownBlock, NotedownRoot};
use wasi_notedown::exports::notedown::core::types::{NotedownError, TextRange};
use crate::utils::{ReadState, ReadMarkdown};


pub struct MarkdownReader {}

impl Default for MarkdownReader {
    fn default() -> Self {
        Self {}
    }
}

impl MarkdownReader {
    pub fn load_str(&self, input: &str) -> Result<NotedownRoot, NotedownError> {
        let arena = Arena::new();
        let mut extension = ComrakExtensionOptions::default();
        extension.math_code = true;
        extension.table = true;
        let mut parse = ComrakParseOptions::default();
        parse.smart = true;
        let options = ComrakOptions {
            extension,
            parse,
            render: Default::default(),
        };
        let mut state = ReadState::default();
        let parsed = parse_document(&arena, input, &options);
        return Ok(NotedownRoot { blocks: parsed.into_note_down(&mut state)?, path: None });
    }
}

enum MaybeInline {
    Block(NotedownBlock),
    Inline(MathSpan),
}

impl<'a> ReadMarkdown<Vec<NotedownBlock>> for &'a Node<'a, RefCell<Ast>> {
    fn into_note_down(self, state: &mut ReadState) -> Result<Vec<NotedownBlock>, NotedownError> {
        let node: NodeValue = self.data.borrow().value.to_owned();
        match node {
            NodeValue::Document => {
                let mut blocks = Vec::with_capacity(4);
                for child in self.children() {
                    blocks.extend(child.into_note_down(state)?)
                }
                Ok(blocks)
            }
            NodeValue::FrontMatter(_) => { unreachable!() }
            NodeValue::BlockQuote => { unreachable!() }
            NodeValue::List(_) => { unreachable!() }
            NodeValue::Item(_) => { unreachable!() }
            NodeValue::DescriptionList => { unreachable!() }
            NodeValue::DescriptionItem(_) => { unreachable!() }
            NodeValue::DescriptionTerm => { unreachable!() }
            NodeValue::DescriptionDetails => { unreachable!() }
            NodeValue::CodeBlock(_) => { unreachable!() }
            NodeValue::HtmlBlock(_) => { unreachable!() }
            NodeValue::Paragraph => {
                let mut blocks = Vec::with_capacity(4);
                for child in self.children() {
                    blocks.extend(child.into_note_down(state)?)
                }
                Ok(blocks)
            }
            NodeValue::Heading(_) => { unreachable!() }
            NodeValue::ThematicBreak => { unreachable!() }
            NodeValue::FootnoteDefinition(_) => { unreachable!() }
            NodeValue::Table(_) => { unreachable!() }
            NodeValue::TableRow(_) => { unreachable!() }
            NodeValue::TableCell => { unreachable!() }
            NodeValue::Text(_) => { unreachable!() }
            NodeValue::TaskItem(_) => { unreachable!() }
            NodeValue::SoftBreak => { unreachable!() }
            NodeValue::LineBreak => { unreachable!() }
            NodeValue::Code(_) => { unreachable!() }
            NodeValue::HtmlInline(_) => { unreachable!() }
            NodeValue::Emph => { unreachable!() }
            NodeValue::Strong => { unreachable!() }
            NodeValue::Strikethrough => { unreachable!() }
            NodeValue::Superscript => { unreachable!() }
            NodeValue::Link(_) => { unreachable!() }
            NodeValue::Image(_) => { unreachable!() }
            NodeValue::FootnoteReference(_) => { unreachable!() }
            NodeValue::ShortCode(_) => { unreachable!() }
            NodeValue::Math(v) => { unreachable!() }
            NodeValue::MultilineBlockQuote(_) => { unreachable!() }
            NodeValue::Escaped => { unreachable!() }
        }
    }
}


impl ReadMarkdown<MathSpan> for NodeMath {
    fn into_note_down(self, _: &mut ReadState) -> Result<MathSpan, NotedownError> {
        let content = MathContent::Tex(self.literal);
        Ok(MathSpan {
            display: MathDisplay::Inline,
            content,
            range: TextRange { head_offset: 0, tail_offset: 0 },
        })
    }
}

#[test]
fn ready() {
    println!("it works!")
}