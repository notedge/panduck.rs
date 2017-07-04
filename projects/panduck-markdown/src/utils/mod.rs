use wasi_notedown::exports::notedown::core::syntax_tree::{NotedownBlock, NotedownRoot, ParagraphTerm};
use wasi_notedown::exports::notedown::core::types::NotedownError;

#[derive(Default)]
pub struct ReadState {
    pub errors: Vec<NotedownError>,
}

pub trait NoteRoot {
    fn note_down_root(self, state: &mut ReadState) -> Result<NotedownRoot, NotedownError>;
}

pub trait NoteBlock {
    fn note_down_block(self, state: &mut ReadState) -> Result<NotedownBlock, NotedownError>;
}

pub trait NoteInline {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphTerm, NotedownError>;
}
