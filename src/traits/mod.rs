use notedown_ast::AST;

pub trait ToNotedown {
    fn to_notedown(&self) -> AST;
}