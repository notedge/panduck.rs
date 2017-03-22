mod html;
mod jupyter;
mod markdown;
mod rich_text;
mod wiki;
mod notedown;
#[cfg(feature = "docx-rs")]
mod docx;

#[cfg(feature = "jupyter")]
pub use jupyter::register_jupyter;

#[cfg(feature = "notedown")]
pub use notedown::register_notedown;

use notedown_parser::{ASTNode};
use std::collections::BTreeSet;


pub type AST = ASTNode<()>;

pub trait ToNotedown {
    fn to_notedown(&self) -> AST;
    fn to_notedown_list(&self) -> Vec<AST> {
        unimplemented!()
    }
}

