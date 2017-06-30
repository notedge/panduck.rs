use comrak::{Arena, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, parse_document};
use comrak::nodes::{AstNode, NodeValue};
use wasi_notedown::exports::notedown::core::syntax_tree::NotedownRoot;
use wasi_notedown::exports::notedown::core::types::NotedownError;
use crate::utils::ReadMarkdown;

pub fn parse_common_markdown(input: &str) -> Result<NotedownRoot, NotedownError> {
    let arena = Arena::new();
    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            tagfilter: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            header_ids: None,
            footnotes: true,
            description_lists: true,
            front_matter_delimiter: Some(String::from("---")),
            multiline_block_quotes: false,
            math_dollars: false,
            math_code: false,
        },
        parse: ComrakParseOptions {
            smart: true,
            default_info_string: None,
            relaxed_tasklist_matching: false,
            relaxed_autolinks: false,
        },
        render: Default::default(),
    };
    let parsed = parse_document(&arena, input, &options);
    return Ok(parsed.into_notedown());
}

impl<'a> ReadMarkdown for &'a AstNode<'a> {
    type Target = ();

    fn into_note_down(self, errors: &mut Vec<NotedownError>) -> Result<Self::Target, NotedownError> {
        let node: NodeValue = self.data.borrow().value.to_owned();
        match node {
            NodeValue::Document => {}
            NodeValue::FrontMatter(_) => {}
            NodeValue::BlockQuote => {}
            NodeValue::List(_) => {}
            NodeValue::Item(_) => {}
            NodeValue::DescriptionList => {}
            NodeValue::DescriptionItem(_) => {}
            NodeValue::DescriptionTerm => {}
            NodeValue::DescriptionDetails => {}
            NodeValue::CodeBlock(_) => {}
            NodeValue::HtmlBlock(_) => {}
            NodeValue::Paragraph => {}
            NodeValue::Heading(_) => {}
            NodeValue::ThematicBreak => {}
            NodeValue::FootnoteDefinition(_) => {}
            NodeValue::Table(_) => {}
            NodeValue::TableRow(_) => {}
            NodeValue::TableCell => {}
            NodeValue::Text(_) => {}
            NodeValue::TaskItem(_) => {}
            NodeValue::SoftBreak => {}
            NodeValue::LineBreak => {}
            NodeValue::Code(_) => {}
            NodeValue::HtmlInline(_) => {}
            NodeValue::Emph => {}
            NodeValue::Strong => {}
            NodeValue::Strikethrough => {}
            NodeValue::Superscript => {}
            NodeValue::Link(_) => {}
            NodeValue::Image(_) => {}
            NodeValue::FootnoteReference(_) => {}
            NodeValue::ShortCode(_) => {}
            NodeValue::Math(_) => {}
            NodeValue::MultilineBlockQuote(_) => {}
            NodeValue::Escaped => {}
        }
        todo!()
    }
}

#[test]
fn ready() {
    println!("it works!")
}