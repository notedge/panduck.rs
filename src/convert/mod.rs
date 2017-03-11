mod html;
mod jupyter;
mod markdown;
mod richtext;
mod wiki;

pub use jupyter::jupyter_from_json;
use crate::TextRange;

pub const EMPTY_RANGE: TextRange = TextRange::default();