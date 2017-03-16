use crate::convert::AST;

pub trait ToNotedown {
    fn to_notedown(&self) -> AST;
    fn to_notedown_list(&self) -> Vec<AST> {
        unimplemented!()
    }
}
