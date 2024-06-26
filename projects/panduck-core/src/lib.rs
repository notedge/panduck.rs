#![feature(box_syntax)]
#![allow(clippy::needless_return)]

pub use notedown_error::{NoteError, Result};
pub use notedown_rt::NoteVM;

// pub use notedown_ast;
pub use convert::ToNotedown;
pub use utils::{ExtensionHandler, ExtensionRegistrar};

pub mod convert;
#[cfg(test)]
pub mod for_test;
pub mod utils;
