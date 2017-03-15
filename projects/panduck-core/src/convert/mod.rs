mod html;
mod jupyter;
mod markdown;
mod rich_text;
mod wiki;

pub use jupyter::jupyter_from_json;
use notedown_parser::{ASTKind, ASTNode};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AST(pub ASTKind<AST>);

impl Default for AST {
    fn default() -> Self {
        Self(ASTKind::default())
    }
}