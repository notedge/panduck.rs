use crate::ToNotedown;
use notedown_ast::ASTNode;
use parse_wiki_text::{Configuration, Node};

fn try_parse() {
    let wiki_text = concat!("==Our values==\n", "*Correctness\n", "*Speed\n", "*Ergonomics");
    let result = Configuration::default().parse(wiki_text);
    assert!(result.warnings.is_empty());
    for node in result.nodes {
        if let Node::UnorderedList { items, .. } = node {
            println!("Our values are:");
            for item in items {
                println!(
                    "- {}",
                    item.nodes
                        .iter()
                        .map(|node| match node {
                            Node::Text { value, .. } => value,
                            _ => "",
                        })
                        .collect::<String>()
                );
            }
        }
    }
}

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
            Node::Text { .. } => {
                unimplemented!()
            }
            Node::UnorderedList { .. } => {
                unimplemented!()
            }
        }
    }
}
