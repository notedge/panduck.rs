use crate::ToNotedown;
use html5ever::rcdom::{RcDom, Handle, Node, NodeEnum};
use notedown_ast::AST;

impl ToNotedown for RcDom {
    fn to_notedown(&self) -> AST {
        self.document.to_notedown()
    }
}


impl ToNotedown for Handle {
    fn to_notedown(&self) -> AST {
        match self.borrow().node {
            NodeEnum::Comment(_) => {}
            NodeEnum::Doctype(_, _, _) => {}
            NodeEnum::Text(ref text) => {
                println!("{:#?}", text);
                unimplemented!()
            },
            NodeEnum::Element(ref name, _, ref attrs) => {
                println!("{:#?}", name);
                println!("{:#?}", self.borrow());
                println!("{:#?}", attrs);
                unimplemented!()
            }
            NodeEnum::Document => {
                for child in self.borrow().children.iter() {
                    println!("{:#?}", child.to_notedown())
                }
            }
        }
        unimplemented!()
    }
}
