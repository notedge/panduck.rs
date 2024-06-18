use crate::utils::{GetTextRange, group_block, group_inline, NoteBlock, NoteBlockList, NoteInline, NoteInlineList, NoteRoot, ReadState};
use markdown::{
    mdast::{
        BlockQuote, Code, Definition, Delete, Emphasis, FootnoteDefinition, FootnoteReference, Heading, Image, InlineCode,
        InlineMath, Link, List, Math, MdxTextExpression, Node, Paragraph,  Strong, Table, Text, Toml, Yaml,
    },
    to_mdast, Constructs, ParseOptions,
};
use std::str::FromStr;
use wasi_notedown::{
    exports::notedown::core::{
        syntax_tree::{
            CodeEnvironment, CodeHighlight, CommandAction, HeadingBlock, ImageReference, LinkReference, ListEnvironment,
            ListItem, MathContent, MathDisplay, MathEnvironment, NormalText, NotedownRoot, ParagraphBlock, ParagraphItem,
            RootItem, StyleType, StyledText, TableCell, TableEnvironment, TableRow,
        },
        types::{NotedownError,  TextRange, Url},
    },
    UrlNative,
};

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
            gfm_strikethrough_single_tilde: true,
            math_text_single_dollar: true,
            mdx_expression_parse: None,
            mdx_esm_parse: None,
        };
        let mut state = ReadState::default();
        let root = match to_mdast(input, &config) {
            Ok(to_mdast) => to_mdast.note_down_root(&mut state),
            Err(e) => {
                todo!("{}", e)
            }
        };
        Ok(root)
    }
}

impl NoteRoot for Node {
    fn note_down_root(self, state: &mut ReadState) -> NotedownRoot {
        let blocks = match self {
            Self::Root(node) => node.children.note_down_block(state),
            _ => unreachable!(),
        };
        NotedownRoot { blocks, config: state.get_object(), path: state.get_path() }
    }
}




#[test]
fn ready() {
    println!("it works!")
}
