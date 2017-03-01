use crate::ToNotedown;
use html_parser::{Dom, DomVariant, Element, Node};
use notedown_ast::{TextRange, AST};

impl ToNotedown for Dom {
    fn to_notedown(&self) -> AST {
        if let DomVariant::Empty = self.tree_type {
            return AST::Statements(vec![]);
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
            "html" | "body" | "main" | "div" | "span" | "article" | "summary" | "details" | "section" | "template" => {
                self.children.to_notedown()
            }
            "head" | "nav" | "meta" | "link" | "script" | "title" | "header" => AST::None,

            "h1" => build_header(1, &self.children, r),
            "h2" => build_header(2, &self.children, r),
            "h3" => build_header(3, &self.children, r),
            "h4" => build_header(4, &self.children, r),
            "h5" => build_header(5, &self.children, r),
            "h6" => build_header(6, &self.children, r),
            "hr" => AST::HorizontalRule { r },

            "p" => AST::Paragraph { children: self.children.to_notedown().to_vec(), r },
            "br" => AST::Normal { inner: String::from("\n"), r },
            "i" | "em" => AST::Italic { children: self.children.to_notedown().to_vec(), r },
            "b" | "strong" => AST::Bold { children: self.children.to_notedown().to_vec(), r },
            "ins" => AST::Underline { children: self.children.to_notedown().to_vec(), r },

            "s" => AST::Strikethrough { children: self.children.to_notedown().to_vec(), r },

            "ul" | "ol" | "blockquote" | "code" | "pre" | "table" | "a" | "img" | "mark" | "sup" | "dl" | "abbr" | "button"
            | "svg" | "form" => {
                // FIXME: fast skip unimplemented
                AST::None
                // unimplemented!("{:?}", self.name)
            }
            _ => {
                if self.name.contains("-") {
                    AST::None
                }
                else {
                    unimplemented!("{:?}", self.name)
                }
            }
        }
    }
}

fn build_header(level: usize, nodes: &Vec<Node>, r: TextRange) -> AST {
    let children = nodes.to_notedown().to_vec();
    match children.len() {
        0 => AST::None,
        _ => AST::Header { level, children, r },
    }
}
