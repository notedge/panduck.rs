use crate::{ExtensionHandler, ExtensionRegistrar, Result, ToNotedown};
use notedown_ast::{ASTKind, ASTNode};
use std::{collections::BTreeSet, iter::FromIterator};
use comrak::{Arena, parse_document, ComrakOptions, ComrakExtensionOptions, ComrakParseOptions};
use comrak::nodes::{AstNode, NodeValue};


pub fn register_common_markdown(r: &mut ExtensionRegistrar) {
    let ext = vec!["markdown", "md"];
    let new = ExtensionHandler {
        name: String::from("markdown"),
        try_extension: BTreeSet::from_iter(ext.into_iter().map(String::from)),
        parser: parse_common_markdown,
    };
    r.insert(new)
}

pub fn parse_common_markdown(input: &str) -> Result<ASTNode>{
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
        parse: ComrakParseOptions {
            smart: true,
            default_info_string: None,
        },
        render: Default::default(),
    };
    let parsed = parse_document(&arena, input, &options);
    return Ok(parsed.into_notedown())
}

impl<'a> ToNotedown for &'a AstNode<'a> {
    fn into_notedown(self) -> ASTNode {
        let node :&NodeValue = &self.data.borrow().value;
        match &node {
            NodeValue::Document => {
                let mut children = vec![];
                for node in self.children() {
                    children.push(node.into_notedown())
                }
                ASTKind::statements(children, None)
            }
            NodeValue::FrontMatter(_)  => {unimplemented!()}
            NodeValue::BlockQuote  => {unimplemented!()}
            NodeValue::List(_)  => {unimplemented!()}
            NodeValue::Item(_)  => {unimplemented!()}
            NodeValue::DescriptionList  => {unimplemented!()}
            NodeValue::DescriptionItem(_)  => {unimplemented!()}
            NodeValue::DescriptionTerm  => {unimplemented!()}
            NodeValue::DescriptionDetails  => {unimplemented!()}
            NodeValue::CodeBlock(_)  => {unimplemented!()}
            NodeValue::HtmlBlock(_)  => {unimplemented!()}
            NodeValue::Paragraph  => {
                let mut children = vec![];
                for node in self.children() {
                    children.push(node.into_notedown())
                }
                ASTKind::paragraph(children, None)
            }
            NodeValue::Heading(_)  => {unimplemented!()}
            NodeValue::ThematicBreak  => {unimplemented!()}
            NodeValue::FootnoteDefinition(_)  => {unimplemented!()}
            NodeValue::Table(_)  => {unimplemented!()}
            NodeValue::TableRow(_)  => {unimplemented!()}
            NodeValue::TableCell  => {unimplemented!()}
            NodeValue::Text(v)  => {
                match String::from_utf8(v.to_owned()) {
                    Ok(o) => {ASTKind::text(o, None)}
                    Err(e) => {panic!("{}" , e)}
                }
            }
            NodeValue::TaskItem(_)  => {unimplemented!()}
            NodeValue::SoftBreak  => {unimplemented!()}
            NodeValue::LineBreak  => {unimplemented!()}
            NodeValue::Code(_)  => {unimplemented!()}
            NodeValue::HtmlInline(_)  => {unimplemented!()}
            NodeValue::Emph  => {unimplemented!()}
            NodeValue::Strong  => {unimplemented!()}
            NodeValue::Strikethrough  => {unimplemented!()}
            NodeValue::Superscript  => {unimplemented!()}
            NodeValue::Link(_)  => {unimplemented!()}
            NodeValue::Image(_)  => {unimplemented!()}
            NodeValue::FootnoteReference(_)  => {unimplemented!()}
        }
    }
}


//
// impl ToNotedown for Vec<Ast> {
//     fn to_notedown(&self) -> ASTNode {
//         ASTKind::statements(self.to_notedown_list(), None)
//     }
//
//     fn to_notedown_list(&self) -> ASTNodes {
//         self.iter().map(ToNotedown::to_notedown).collect()
//     }
// }
//
// impl ToNotedown for Block {
//     fn to_notedown(&self) -> ASTNode {
//         match self {
//             Block::Header(content, level) => ASTKind::header(content.to_notedown_list(), *level as u8, None),
//             Block::Paragraph(p) => ASTKind::paragraph(p.to_notedown_list(), None),
//             Block::CodeBlock(lang, code) => {
//                 let lang = match lang {
//                     Some(s) => { s.as_str() }
//                     None => { "text" }
//                 };
//                 ASTKind::code_block(lang, code, None)
//             }
//             Block::Raw(_) => unimplemented!(),
//             Block::Hr => unimplemented!(),
//             // Block::Blockquote(list) => AST::QuoteList { style: None, body: list.to_notedown().to_vec(), r },
//             // Block::OrderedList(list, _) => AST::OrderedList { head: 1, body: list.to_notedown().to_vec(), r },
//             // Block::UnorderedList(list) => AST::OrderlessList { body: list.to_notedown().to_vec(), r },
//             Block::Blockquote(_) => unimplemented!(),
//             Block::OrderedList(_, _) => unimplemented!(),
//             Block::UnorderedList(_) => unimplemented!(),
//         }
//     }
// }
//
// impl ToNotedown for Vec<Span> {
//     fn to_notedown(&self) -> ASTNode {
//         ASTKind::statements(self.to_notedown_list(), None)
//     }
//
//     fn to_notedown_list(&self) -> ASTNodes {
//         let mut out = vec![];
//         for node in self {
//             let ast = node.to_notedown();
//             match ast.value {
//                 ASTKind::Statements(v) => out.extend(v),
//                 _ => out.push(ast),
//             }
//         }
//         return out;
//     }
// }
//
// impl ToNotedown for Span {
//     fn to_notedown(&self) -> ASTNode {
//         match self {
//             Span::Break => unimplemented!(),
//             Span::Text(t) => ASTKind::text(t, None),
//             Span::Code(code) => ASTKind::code_inline(code, None),
//             Span::Link(text, url, title) => {
//                 unimplemented!()
//                 // let link = SmartLink::Hyperlinks {
//                 //     from: text.into(),
//                 //     to: Some(url.into()),
//                 //     alt: title.as_ref().map(Into::into),
//                 //     bind: None,
//                 // };
//                 // AST::link { inner: link, r }
//             }
//             Span::Image(_, _, _) => unimplemented!(),
//             Span::Emphasis(_) => unimplemented!(),
//             Span::Strong(children) => ASTKind::strong(children.to_notedown_list(), None),
//         }
//     }
// }
//
// impl ToNotedown for Vec<ListItem> {
//     fn to_notedown(&self) -> ASTNode {
//         ASTKind::statements(self.iter().map(ToNotedown::to_notedown).collect(), None)
//     }
// }
//
// impl ToNotedown for ListItem {
//     fn to_notedown(&self) -> ASTNode {
//         match self {
//             ListItem::Simple(s) => s.to_notedown(),
//             ListItem::Paragraph(p) => ASTKind::paragraph(p.to_notedown_list(), None),
//         }
//     }
// }
