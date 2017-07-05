use markdown::unist::Position;
use wasi_notedown::exports::notedown::core::syntax_tree::{NotedownRoot, ParagraphItem, RootItem};
use wasi_notedown::exports::notedown::core::types::{NotedownError, TextRange};

#[derive(Default)]
pub struct ReadState {
    pub errors: Vec<NotedownError>,
}

pub trait NoteRoot {
    fn note_down_root(self, state: &mut ReadState) -> Result<NotedownRoot, NotedownError>;
}

pub trait NoteBlock {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError>;
}

pub trait NoteInline {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError>;
}

pub trait GetTextRange {
    fn as_range(&self) -> TextRange;
}

impl GetTextRange for Option<Position> {
    fn as_range(&self) -> TextRange {
        match self {
            Some(s) => {
                TextRange {
                    head_offset: s.start.offset as u32,
                    tail_offset: s.end.offset as u32,
                }
            }
            None => {
                TextRange {
                    head_offset: 0,
                    tail_offset: 0,
                }
            }
        }
    }
}