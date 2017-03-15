use crate::ToNotedown;
use html_parser::{Dom, DomVariant, Element, Node};
use notedown_parser::{ASTKind};
use crate::convert::{ AST};



impl ToNotedown for Dom {
    fn to_notedown(&self) -> AST {
        if let DomVariant::Empty = self.tree_type {
            return AST(ASTKind::statements(vec![]))
        }
        self.children.to_notedown()
    }
}

impl ToNotedown for Vec<Node> {
    fn to_notedown(&self) -> AST {
        let mut out = vec![];
        for node in self {
            let ast = node.to_notedown();
            match ast.0 {
                ASTKind::None => continue,
                ASTKind::Statements(v) => out.extend(v),
                _ => out.push(ast),
            }
        }
        return AST(ASTKind::statements(out));
    }
}

impl ToNotedown for Node {
    fn to_notedown(&self) -> AST {
        match self {
            Node::Text(s) => AST(ASTKind::text(s.to_owned(),"text",)),
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
        let kind = match self.name.as_str() {
            "html" | "body" | "main" | "div" | "span" | "article" | "summary" | "details" | "section" | "template" => {
                self.children.to_notedown()
            }
            "head" | "nav" | "meta" | "link" | "script" | "title" | "header" => AST::default(),
            "h1" => AST(ASTKind::header( self.children.to_notedown().children(), 1)),
            "h2" => ASTKind::header( self.children.to_notedown().children(), 2,),
            "h3" => ASTKind::header( self.children.to_notedown().children(), 3, ),
            "h4" => ASTKind::header( self.children.to_notedown().children(), 4, ),
            "h5" => ASTKind::header( self.children.to_notedown().children(), 5, ),
            "h6" => ASTKind::header( self.children.to_notedown().children(), 6, ),
            "hr" => ASTKind::HorizontalRule,
            "p" => ASTKind::paragraph(self.children.to_notedown().children(),),
            "br" => ASTKind::Normal(Box::new(String::from("\n"))),
            "i" | "em" => ASTKind::style(self.children.to_notedown().children(), "*",  ),
            "b" | "strong" => ASTKind::style(self.children.to_notedown().children(), "**",  ),
            "ins" => ASTKind::style(self.children.to_notedown().children(), "~",  ),
            "s" => ASTKind::style(self.children.to_notedown().children(), "~~",  ),
            "ul" | "ol" | "blockquote" | "code" | "pre" | "table" | "a" | "img" | "mark" | "sup" | "dl" | "abbr" | "button"
            | "svg" | "form" => {
                // FIXME: fast skip unimplemented
                ASTKind::default()
                // unimplemented!("{:?}", self.name)
            }
            _ => {
                if self.name.contains("-") {
                    ASTKind::default()
                }
                else {
                    unimplemented!("{:?}", self.name)
                }
            }
        };
    }
}
