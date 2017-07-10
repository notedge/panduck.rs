use crate::utils::{paragraph_items, root_items, GetTextRange, NoteBlock, NoteInline, NoteRoot, ReadState};
use markdown::{
    mdast::{
        BlockQuote, Code, Delete, Emphasis, InlineCode, InlineMath, List, Math, Node, Paragraph, Root, Strong, Table, Text,
    },
    to_mdast, Constructs, ParseOptions,
};
use wasi_notedown::exports::notedown::core::{
    syntax_tree::{
        CodeAction, CodeEnvironment, CodeHighlight, ListEnvironment, ListItem, MathContent, MathDisplay, MathEnvironment,
        NormalText, NotedownRoot, ParagraphBlock, ParagraphItem, RootItem, StyleType, StyledText, TableCell, TableEnvironment,
        TableRow,
    },
    types::{NotedownError, TextRange},
};
use wasi_notedown::exports::notedown::core::types::Object;

mod blocks;
mod html;
mod inline;
mod list;
mod table;

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
            _ => unreachable!(),
        }
    }
}

impl NoteRoot for Root {
    fn note_down_root(self, state: &mut ReadState) -> Result<NotedownRoot, NotedownError> {
        let blocks = root_items(self.children, state)?;
        Ok(NotedownRoot { blocks, config: Object { map: vec![] }, path: None })
    }
}

#[test]
fn ready() {
    println!("it works!")
}
