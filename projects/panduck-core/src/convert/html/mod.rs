use crate::{ToNotedown, ExtensionRegistrar, ExtensionHandler};
use html_parser::{Dom, DomVariant, Element, Node};
use notedown_ast::{ASTKind, ASTNode, ASTNodes};
use crate::Result;
use std::collections::BTreeSet;
use std::iter::FromIterator;

pub fn register_html(r: &mut ExtensionRegistrar) {
    let ext = vec!["xhtml", "html"];
    let new = ExtensionHandler {
        name: String::from("html"),
        try_extension: BTreeSet::from_iter(ext.into_iter().map(String::from)),
        parser: parse_html,
    };
    r.insert(new)
}

pub fn parse_html(text: &str) -> Result<ASTNode> {
    Ok(Dom::parse(text)?.to_notedown())
}

impl ToNotedown for Dom {
    fn to_notedown(&self) -> ASTNode {
        if let DomVariant::Empty = self.tree_type {
            return ASTKind::statements(vec![], None);
        }
        self.children.to_notedown()
    }
}

impl ToNotedown for Vec<Node> {
    fn to_notedown(&self) -> ASTNode {
        ASTKind::statements(self.to_notedown_list(), None)
    }

    fn to_notedown_list(&self) -> ASTNodes {
        let mut out = vec![];
        for node in self {
            let ast = node.to_notedown();
            match ast.kind {
                ASTKind::None => continue,
                ASTKind::Statements(v) => out.extend(v),
                _ => out.push(ast),
            }
        }
        return out;
    }
}

impl ToNotedown for Node {
    fn to_notedown(&self) -> ASTNode {
        match self {
            Node::Text(s) => ASTNode { kind: ASTKind::text(s.to_owned(), "text"), meta: () },
            Node::Comment(s) => {
                println!("{:?}", s);
                unimplemented!()
            }
            Node::Element(e) => e.to_notedown(),
        }
    }
}

impl ToNotedown for Element {
    fn to_notedown(&self) -> ASTNode {
        match self.name.as_str() {
            "html" | "body" | "main" | "div" | "span" | "article" | "summary" | "details" | "section" | "template" => {
                self.children.to_notedown()
            }
            "head" | "nav" | "meta" | "link" | "script" | "title" | "header" => ASTNode::default(),
            "h1" => ASTKind::header(self.children.to_notedown_list(), 1),
            "h2" => ASTKind::header(self.children.to_notedown_list(), 2),
            "h3" => ASTKind::header(self.children.to_notedown_list(), 3),
            "h4" => ASTKind::header(self.children.to_notedown_list(), 4),
            "h5" => ASTKind::header(self.children.to_notedown_list(), 5),
            "h6" => ASTKind::header(self.children.to_notedown_list(), 6),
            "hr" => ASTKind::hr(None),
            "p" => ASTKind::paragraph(self.children.to_notedown_list()),
            "br" => ASTKind::text("\n".to_string(), ""),
            "i" | "em" => ASTKind::style(self.children.to_notedown_list(), "*"),
            "b" | "strong" => ASTKind::style(self.children.to_notedown_list(), "**"),
            "ins" => ASTKind::style(self.children.to_notedown_list(), "~"),
            "s" => ASTKind::style(self.children.to_notedown_list(), "~~"),
            "ul" | "ol" | "blockquote" | "code" | "pre" | "table" | "a" | "img" | "mark" | "sup" | "dl" | "abbr" | "button"
            | "svg" | "form" => {
                // FIXME: fast skip unimplemented
                ASTNode::default()
                // unimplemented!("{:?}", self.name)
            }
            _ => {
                if self.name.contains("-") {
                    ASTNode::default()
                } else {
                    unimplemented!("{:?}", self.name)
                }
            }
        }
    }
}
