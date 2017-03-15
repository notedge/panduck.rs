use crate::convert::AST;

pub trait ToNotedown {
    fn to_notedown(&self) -> AST;
}
