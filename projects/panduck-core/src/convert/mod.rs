#[cfg(feature = "docx-rs")]
mod docx;
#[cfg(feature = "html")]
mod html;
#[cfg(feature = "jupyter")]
mod jupyter;
#[cfg(feature = "markdown")]
mod markdown;
#[cfg(feature = "notedown")]
mod notedown;
#[cfg(feature = "rtf")]
mod rich_text;
#[cfg(feature = "wiki")]
mod wiki;

#[cfg(feature = "jupyter")]
pub use jupyter::register_jupyter;

#[cfg(feature = "notedown")]
pub use notedown::register_notedown;

use notedown_ast::{ASTNode, ASTNodes};

pub trait ToNotedown {
    fn to_notedown(&self) -> ASTNode;
    fn to_notedown_list(&self) -> ASTNodes {
        vec![self.to_notedown()]
    }
}
