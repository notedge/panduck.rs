use crate::ToNotedown;
use html_parser::{Dom, DomVariant, Element, Node};
use notedown_parser::{TextRange, AST};
use crate::convert::EMPTY_RANGE;

impl ToNotedown for Dom {
    fn to_notedown(&self) -> AST {
        if let DomVariant::Empty = self.tree_type {
            return AST::statements(vec![],EMPTY_RANGE);
        }
        self.children.to_notedown()
    }
}

impl ToNotedown for Vec<Node> {
    fn to_notedown(&self) -> AST {
        let mut out = vec![];
        for node in self {
            let ast = node.to_notedown();
            match ast {
                AST::None => continue,
                AST::Statements(v) => out.extend(v),
                _ => out.push(ast),
            }
        }
        return AST::statements(out, EMPTY_RANGE);
    }
}

impl ToNotedown for Node {
    fn to_notedown(&self) -> AST {
        match self {
            Node::Text(s) => AST::text(s.to_owned(),"text",EMPTY_RANGE),
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
            "h1" => AST::header( self.children.to_notedown().children(), 1, EMPTY_RANGE),
            "h2" => AST::header( self.children.to_notedown().children(), 2, EMPTY_RANGE),
            "h3" => AST::header( self.children.to_notedown().children(), 3, EMPTY_RANGE),
            "h4" => AST::header( self.children.to_notedown().children(), 4, EMPTY_RANGE),
            "h5" => AST::header( self.children.to_notedown().children(), 5, EMPTY_RANGE),
            "h6" => AST::header( self.children.to_notedown().children(), 6, EMPTY_RANGE),
            "hr" => AST::HorizontalRule { r },
            "p" => AST::paragraph(self.children.to_notedown().children(),EMPTY_RANGE),
            "br" => AST::Normal { inner: String::from("\n"), r },
            "i" | "em" => AST::style(self.children.to_notedown().children(), "*", EMPTY_RANGE ),
            "b" | "strong" => AST::style(self.children.to_notedown().children(), "**", EMPTY_RANGE ),
            "ins" => AST::style(self.children.to_notedown().children(), "~", EMPTY_RANGE ),
            "s" => AST::style(self.children.to_notedown().children(), "~~", EMPTY_RANGE ),
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
