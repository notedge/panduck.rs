use crate::ToNotedown;
use html_parser::{Dom, DomVariant, Element, Node};
use notedown_ast::AST;

impl ToNotedown for Dom {
    fn to_notedown(&self) -> AST {
        match self.tree_type {
            DomVariant::Document => println!("{:?}", self.tree_type),
            DomVariant::DocumentFragment => println!("{:?}", self.tree_type),
            DomVariant::Empty => return AST::Statements(vec![]),
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
                AST::Statements(v) => out.extend(v),
                _ => out.push(ast),
            }
        }
        return AST::Statements(out);
    }
}

impl ToNotedown for Node {
    fn to_notedown(&self) -> AST {
        match self {
            Node::Text(s) => AST::Normal { inner: s.to_owned(), r: Default::default() },
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
        let r = Default::default();
        match self.name.as_str() {
            "div" | "section" => self.children.to_notedown(),

            "h1" => AST::Header { level: 1, children: self.children.to_notedown().to_vec(), r },
            "h2" => AST::Header { level: 2, children: self.children.to_notedown().to_vec(), r },
            "h3" => AST::Header { level: 3, children: self.children.to_notedown().to_vec(), r },
            "h4" => AST::Header { level: 4, children: self.children.to_notedown().to_vec(), r },
            "h5" => AST::Header { level: 5, children: self.children.to_notedown().to_vec(), r },
            "h6" => AST::Header { level: 6, children: self.children.to_notedown().to_vec(), r },
            "hr" => AST::HorizontalRule { r },

            "p" => AST::Paragraph { children: self.children.to_notedown().to_vec(), r },
            "i" | "em" => AST::Italic { children: self.children.to_notedown().to_vec(), r },
            "b" | "strong" => AST::Bold { children: self.children.to_notedown().to_vec(), r },

            "s" => AST::Strikethrough { children: self.children.to_notedown().to_vec(), r },

            "ul" | "ol" | "blockquote" | "code" | "pre" | "table" | "a" | "img" | "ins" | "mark" | "sup" | "dl" | "abbr" => {
                // FIXME: fast skip unimplemented
                AST::None
                // unimplemented!("{:?}", self.name)
            }
            _ => unimplemented!("{:?}", self.name),
        }
    }
}
