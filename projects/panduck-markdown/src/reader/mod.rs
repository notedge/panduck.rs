use std::cell::RefCell;
use markdown::{Constructs, ParseOptions, to_mdast};
use markdown::mdast::Node;

use wasi_notedown::exports::notedown::core::syntax_tree::{MathContent, MathDisplay, MathSpan, NotedownBlock, NotedownRoot};
use wasi_notedown::exports::notedown::core::types::{NotedownError, TextRange};
use crate::utils::{ReadState, ReadMarkdown};


pub struct MarkdownParser {}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self {}
    }
}

impl MarkdownParser {
    pub fn load_str(&self, input: &str) -> Result<NotedownRoot, Vec<NotedownError>> {
        let config = ParseOptions {
            constructs: Constructs {
                attention: true,
                autolink: true,
                block_quote: true,
                character_escape: true,
                character_reference: true,
                code_indented: true,
                code_fenced: true,
                code_text: true,
                definition: true,
                frontmatter: true,
                gfm_autolink_literal: true,
                gfm_footnote_definition: true,
                gfm_label_start_footnote: true,
                gfm_strikethrough: true,
                gfm_table: true,
                gfm_task_list_item: true,
                hard_break_escape: true,
                hard_break_trailing: true,
                heading_atx: true,
                heading_setext: true,
                html_flow: true,
                html_text: true,
                label_start_image: true,
                label_start_link: true,
                label_end: true,
                list_item: true,
                math_flow: true,
                math_text: true,
                mdx_esm: true,
                mdx_expression_flow: true,
                mdx_expression_text: true,
                mdx_jsx_flow: true,
                mdx_jsx_text: true,
                thematic_break: true,
            },
            gfm_strikethrough_single_tilde: false,
            math_text_single_dollar: false,
            mdx_expression_parse: None,
            mdx_esm_parse: None,
        };
        let mut state = ReadState::default();
        let root = match to_mdast(input, &config) {
            Ok(to_mdast) => {
                match to_mdast.into_note_down(&mut state) {
                    Ok(o) => { o }
                    Err(e) => {
                        todo!()
                    }
                }
            }
            Err(e) => { todo!() }
        };
        Ok(root)
    }
}

enum MaybeInline {
    Block(NotedownBlock),
    Inline(MathSpan),
}

impl ReadMarkdown<NotedownRoot> for Node {
    fn into_note_down(self, state: &mut ReadState) -> Result<NotedownRoot, NotedownError> {
        match self {
            Node::Root(v) => {
                todo!()
            }
            Node::BlockQuote(_) => { todo!() }
            Node::FootnoteDefinition(_) => { todo!() }
            Node::MdxJsxFlowElement(_) => { todo!() }
            Node::List(_) => { todo!() }
            Node::MdxjsEsm(_) => { todo!() }
            Node::Toml(_) => { todo!() }
            Node::Yaml(_) => { todo!() }
            Node::Break(_) => { todo!() }
            Node::InlineCode(_) => { todo!() }
            Node::InlineMath(_) => { todo!() }
            Node::Delete(_) => { todo!() }
            Node::Emphasis(_) => { todo!() }
            Node::MdxTextExpression(_) => { todo!() }
            Node::FootnoteReference(_) => { todo!() }
            Node::Html(_) => { todo!() }
            Node::Image(_) => { todo!() }
            Node::ImageReference(_) => { todo!() }
            Node::MdxJsxTextElement(_) => { todo!() }
            Node::Link(_) => { todo!() }
            Node::LinkReference(_) => { todo!() }
            Node::Strong(_) => { todo!() }
            Node::Text(_) => { todo!() }
            Node::Code(_) => { todo!() }
            Node::Math(_) => { todo!() }
            Node::MdxFlowExpression(_) => { todo!() }
            Node::Heading(_) => { todo!() }
            Node::Table(_) => { todo!() }
            Node::ThematicBreak(_) => { todo!() }
            Node::TableRow(_) => { todo!() }
            Node::TableCell(_) => { todo!() }
            Node::ListItem(_) => { todo!() }
            Node::Definition(_) => { todo!() }
            Node::Paragraph(_) => { todo!() }
        }
    }
}

//
// impl ReadMarkdown<MathSpan> for NodeMath {
//     fn into_note_down(self, _: &mut ReadState) -> Result<MathSpan, NotedownError> {
//         let content = MathContent::Tex(self.literal);
//         Ok(MathSpan {
//             display: MathDisplay::Inline,
//             content,
//             range: TextRange { head_offset: 0, tail_offset: 0 },
//         })
//     }
// }

#[test]
fn ready() {
    println!("it works!")
}