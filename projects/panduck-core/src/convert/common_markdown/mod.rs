mod code;
mod html;
mod list;
mod table;
mod link;

use crate::{ExtensionHandler, ExtensionRegistrar, Result, ToNotedown};
use comrak::{
    arena_tree::Children,
    nodes::{Ast, AstNode, NodeValue},
    parse_document, Arena, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions,
};
use notedown_ast::{ASTKind, ASTNode, ASTNodes};
use std::{cell::RefCell, collections::BTreeSet, iter::FromIterator};

pub fn register_common_markdown(r: &mut ExtensionRegistrar) {
    let ext = vec!["markdown", "md"];
    let new = ExtensionHandler {
        name: String::from("markdown"),
        try_extension: BTreeSet::from_iter(ext.into_iter().map(String::from)),
        parser: parse_common_markdown,
    };
    r.insert(new)
}

pub fn parse_common_markdown(input: &str) -> Result<ASTNode> {
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
            front_matter_delimiter: None,
        },
        parse: ComrakParseOptions { smart: true, default_info_string: None },
        render: Default::default(),
    };
    let parsed = parse_document(&arena, input, &options);
    return Ok(parsed.into_notedown());
}

impl<'a> ToNotedown for &'a AstNode<'a> {
    fn into_notedown(self) -> ASTNode {
        let node: NodeValue = self.data.borrow().value.to_owned();
        match node {
            NodeValue::Document => ASTKind::statements(self.children().into_notedown_list(), None),
            NodeValue::FrontMatter(_) => {
                unimplemented!()
            }
            NodeValue::BlockQuote => {
                ASTNode::default()
            }
            NodeValue::List(v) => v.into_notedown(),
            NodeValue::Item(_) => {
                unimplemented!()
            }
            NodeValue::DescriptionList => {
                ASTNode::default()
            }
            NodeValue::DescriptionItem(_) => {
                unimplemented!()
            }
            NodeValue::DescriptionTerm => {
                unimplemented!()
            }
            NodeValue::DescriptionDetails => {
                unimplemented!()
            }
            NodeValue::CodeBlock(v) => v.into_notedown(),
            NodeValue::HtmlBlock(v) => v.into_notedown(),
            NodeValue::Paragraph => ASTKind::paragraph(self.children().into_notedown_list(), None),
            NodeValue::Heading(v) => ASTKind::header(self.children().into_notedown_list(), v.level as u8, None),
            NodeValue::ThematicBreak => ASTKind::hr(None),
            NodeValue::FootnoteDefinition(_) => {
                unimplemented!()
            }
            NodeValue::Table(v) => v.into_notedown(),
            NodeValue::TableRow(_) => {
                unimplemented!()
            }
            NodeValue::TableCell => {
                unimplemented!()
            }
            NodeValue::Text(v) => match String::from_utf8(v.to_owned()) {
                Ok(o) => ASTKind::text(o, None),
                Err(e) => {
                    panic!("{}", e)
                }
            },
            NodeValue::TaskItem(_) => {
                unimplemented!()
            }
            NodeValue::SoftBreak => {
                ASTKind::soft_break(None)
            }
            NodeValue::LineBreak => ASTKind::hard_break(None),
            NodeValue::Code(v) => v.into_notedown(),
            NodeValue::HtmlInline(_) => {
                unimplemented!()
            }
            NodeValue::Emph => ASTKind::emphasis(self.children().into_notedown_list(), None),
            NodeValue::Strong => ASTKind::strong(self.children().into_notedown_list(), None),
            NodeValue::Strikethrough => ASTKind::delete(self.children().into_notedown_list(), None),
            NodeValue::Superscript => {
                unimplemented!()
            }
            NodeValue::Link(v) => v.into_notedown(),
            NodeValue::Image(v) => v.into_notedown(),
            NodeValue::FootnoteReference(_) => {
                unimplemented!()
            }
        }
    }
}

impl ToNotedown for Children<'_, RefCell<Ast>> {
    fn into_notedown(self) -> ASTNode {
        todo!()
    }

    fn into_notedown_list(self) -> ASTNodes {
        let mut children = vec![];
        for node in self {
            children.push(node.into_notedown())
        }
        return children;
    }
}

