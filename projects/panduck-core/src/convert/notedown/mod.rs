use crate::{ToNotedown, AST};
use notedown_parser::{ASTNode, ASTKind};
use notedown_parser::utils::LSPMetaInfo;

impl ToNotedown for notedown_parser::AST {
    fn to_notedown(&self) -> AST {
        self.kind.to_notedown()
    }
}

impl ToNotedown for ASTKind<ASTNode<LSPMetaInfo>> {
    fn to_notedown(&self) -> AST {
        match self {
            ASTKind::None => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Statements(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Header(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::HorizontalRule => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Paragraph(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::CodeBlock(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::MathBlock(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::TableView(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::ListView(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Normal(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Raw(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Code(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Italic(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Bold(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Emphasis(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Underline(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Strikethrough(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Undercover(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::MathInline(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::MathDisplay(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Escaped(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Link(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Command(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::String(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Number(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Boolean(_) => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Array => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
            ASTKind::Object => {AST {
                kind: ASTKind::None,
                meta: ()
            }}
        }
    }
}