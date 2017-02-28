use crate::ToNotedown;
use notedown_ast::AST;
use html5ever::rcdom::RcDom;

impl ToNotedown for RcDom {
    fn to_notedown(&self) -> AST {
        unimplemented!()
    }
}

