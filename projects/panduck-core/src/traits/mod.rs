use notedown_parser::AST;

pub trait ToNotedown {
    fn to_notedown(&self) -> AST;
}
