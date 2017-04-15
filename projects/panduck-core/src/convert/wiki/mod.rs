use crate::ToNotedown;
use notedown_ast::{ASTKind, ASTNode};
use parse_wiki_text::{Configuration, Node};
use std::ops::Range;

impl ToNotedown for Node {
    fn into_notedown(self) -> ASTNode {
        match self {
            Node::Bold { .. } => {
                unimplemented!()
            }
            Node::BoldItalic { .. } => {
                unimplemented!()
            }
            Node::Category { .. } => {
                unimplemented!()
            }
            Node::CharacterEntity { .. } => {
                unimplemented!()
            }
            Node::Comment { .. } => {
                unimplemented!()
            }
            Node::DefinitionList { .. } => {
                unimplemented!()
            }
            Node::EndTag { .. } => {
                unimplemented!()
            }
            Node::ExternalLink { .. } => {
                unimplemented!()
            }
            Node::Heading { .. } => {
                unimplemented!()
            }
            Node::HorizontalDivider { .. } => {
                unimplemented!()
            }
            Node::Image { .. } => {
                unimplemented!()
            }
            Node::Italic { .. } => {
                unimplemented!()
            }
            Node::Link { .. } => {
                unimplemented!()
            }
            Node::MagicWord { .. } => {
                unimplemented!()
            }
            Node::OrderedList { .. } => {
                unimplemented!()
            }
            Node::ParagraphBreak { .. } => {
                unimplemented!()
            }
            Node::Parameter { .. } => {
                unimplemented!()
            }
            Node::Preformatted { .. } => {
                unimplemented!()
            }
            Node::Redirect { .. } => {
                unimplemented!()
            }
            Node::StartTag { .. } => {
                unimplemented!()
            }
            Node::Table { .. } => {
                unimplemented!()
            }
            Node::Tag { .. } => {
                unimplemented!()
            }
            Node::Template { .. } => {
                unimplemented!()
            }
            Node::Text { end, start, value } => ASTKind::text(value, Some(Range { start, end })),
            Node::UnorderedList { .. } => {
                unimplemented!()
            }
        }
    }
}
