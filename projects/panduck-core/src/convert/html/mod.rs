use crate::{convert::AST, ToNotedown};
use html_parser::{Dom, DomVariant, Element, Node};
use notedown_parser::{ASTKind, ASTNode};

pub fn parse_html(text: &str) -> Result<AST> {
    Ok(Dom::parse(text)?.to_notedown())
}

impl ToNotedown for Dom {
    fn to_notedown(&self) -> AST {
        if let DomVariant::Empty = self.tree_type {
            return AST::statements(vec![]);
        }
        self.children.to_notedown()
    }
}

impl ToNotedown for Vec<Node> {
    fn to_notedown(&self) -> AST {
        AST::statements(self.to_notedown_list())
    }

    fn to_notedown_list(&self) -> Vec<AST> {
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
    fn to_notedown(&self) -> AST {
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
    fn to_notedown(&self) -> AST {
        match self.name.as_str() {
            "html" | "body" | "main" | "div" | "span" | "article" | "summary" | "details" | "section" | "template" => {
                self.children.to_notedown()
            }
            "head" | "nav" | "meta" | "link" | "script" | "title" | "header" => AST::default(),
            "h1" => AST::header(self.children.to_notedown_list(), 1),
            "h2" => AST::header(self.children.to_notedown_list(), 2),
            "h3" => AST::header(self.children.to_notedown_list(), 3),
            "h4" => AST::header(self.children.to_notedown_list(), 4),
            "h5" => AST::header(self.children.to_notedown_list(), 5),
            "h6" => AST::header(self.children.to_notedown_list(), 6),
            "hr" => AST::hr(),
            "p" => AST::paragraph(self.children.to_notedown_list()),
            "br" => AST::text("\n".to_string(), ""),
            "i" | "em" => AST::style(self.children.to_notedown_list(), "*"),
            "b" | "strong" => AST::style(self.children.to_notedown_list(), "**"),
            "ins" => AST::style(self.children.to_notedown_list(), "~"),
            "s" => AST::style(self.children.to_notedown_list(), "~~"),
            "ul" | "ol" | "blockquote" | "code" | "pre" | "table" | "a" | "img" | "mark" | "sup" | "dl" | "abbr" | "button"
            | "svg" | "form" => {
                // FIXME: fast skip unimplemented
                AST::default()
                // unimplemented!("{:?}", self.name)
            }
            _ => {
                if self.name.contains("-") {
                    AST::default()
                }
                else {
                    unimplemented!("{:?}", self.name)
                }
            }
        }
    }
}
