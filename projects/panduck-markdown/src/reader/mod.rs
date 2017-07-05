use crate::utils::{NoteBlock, NoteInline, NoteRoot, ReadState};
use markdown::{
    mdast::{Math, Node, Paragraph, Root, Text},
    to_mdast, Constructs, ParseOptions,
};
use wasi_notedown::exports::notedown::core::{
    syntax_tree::{MathContent, MathDisplay, MathSpan, NotedownRoot, ParagraphBlock, ParagraphItem, RootItem},
    types::{NotedownError, TextRange},
};

mod old;
mod code;

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
            Ok(to_mdast) => match to_mdast.note_down_root(&mut state) {
                Ok(o) => o,
                Err(e) => {
                    todo!()
                }
            },
            Err(e) => {
                todo!()
            }
        };
        Ok(root)
    }
}

impl NoteRoot for Node {
    fn note_down_root(self, state: &mut ReadState) -> Result<NotedownRoot, NotedownError> {
        match self {
            Self::Root(node) => node.note_down_root(state),
            _ => unreachable!("{self:?}"),
        }
    }
}



impl NoteInline for Node {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        match self {
            Node::Root(_) => {
                todo!()
            }
            Node::BlockQuote(_) => {
                todo!()
            }
            Node::FootnoteDefinition(_) => {
                todo!()
            }
            Node::MdxJsxFlowElement(_) => {
                todo!()
            }
            Node::List(_) => {
                todo!()
            }
            Node::MdxjsEsm(_) => {
                todo!()
            }
            Node::Toml(_) => {
                todo!()
            }
            Node::Yaml(_) => {
                todo!()
            }
            Node::Break(_) => {
                todo!()
            }
            Node::InlineCode(v) => v.note_down_inline(state),
            Node::InlineMath(_) => {
                todo!()
            }
            Node::Delete(v) => v.note_down_inline(state),
            Node::Emphasis(v) => v.note_down_inline(state),
            Node::MdxTextExpression(_) => {
                todo!()
            }
            Node::FootnoteReference(_) => {
                todo!()
            }
            Node::Html(_) => {
                todo!()
            }
            Node::Image(_) => {
                todo!()
            }
            Node::ImageReference(_) => {
                todo!()
            }
            Node::MdxJsxTextElement(_) => {
                todo!()
            }
            Node::Link(_) => {
                todo!()
            }
            Node::LinkReference(_) => {
                todo!()
            }
            Node::Strong(v) => v.note_down_inline(state),
            Node::Text(v) => v.note_down_inline(state),
            Node::Code(_) => {
                todo!()
            }
            Node::Math(_) => {
                todo!()
            }
            Node::MdxFlowExpression(_) => {
                todo!()
            }
            Node::Heading(_) => {
                todo!()
            }
            Node::Table(_) => {
                todo!()
            }
            Node::ThematicBreak(_) => {
                todo!()
            }
            Node::TableRow(_) => {
                todo!()
            }
            Node::TableCell(_) => {
                todo!()
            }
            Node::ListItem(_) => {
                todo!()
            }
            Node::Definition(_) => {
                todo!()
            }
            Node::Paragraph(v) => {
                todo!()
            }
        }
    }
}



impl NoteRoot for Root {
    fn note_down_root(self, state: &mut ReadState) -> Result<NotedownRoot, NotedownError> {
        let mut blocks = Vec::with_capacity(self.children.len());
        for x in self.children {
            match x.note_down_block(state) {
                Ok(o) => blocks.push(o),
                Err(e) => {
                    state.errors.push(e);
                }
            }
        }
        Ok(NotedownRoot { blocks, path: None })
    }
}

#[test]
fn ready() {
    println!("it works!")
}
