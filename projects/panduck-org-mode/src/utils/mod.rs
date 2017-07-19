use wasi_notedown::exports::notedown::core::{
    syntax_tree::{NotedownRoot, ParagraphItem, RootItem},
    types::{NotedownError, TextRange},
};

#[derive(Default)]
pub struct ReadState {
    errors: Vec<NotedownError>,
}

impl ReadState {
    pub fn note_error(&mut self, e: NotedownError) {
        self.errors.push(e)
    }
    pub fn take_errors<T>(&mut self, r: Result<T, NotedownError>) -> Option<T> {
        match r {
            Ok(o) => Some(o),
            Err(e) => {
                self.note_error(e);
                None
            }
        }
    }
    pub fn finish(mut self, result: Result<NotedownRoot, NotedownError>) -> Result<NotedownRoot, Vec<NotedownError>> {
        match result {
            Ok(o) => {
                if self.errors.is_empty() {
                    Ok(o)
                }
                else {
                    Err(self.errors)
                }
            }
            Err(e) => {
                self.note_error(e);
                Err(self.errors)
            }
        }
    }
}

pub trait NoteRoot {
    fn note_down_root(self, state: &mut ReadState) -> Result<NotedownRoot, NotedownError>;
}

pub trait NoteBlock {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError>;
}
pub trait NoteBlockList {
    fn note_down_block(self, state: &mut ReadState) -> Vec<RootItem>;
}

pub trait NoteInline {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError>;
}
pub trait NoteInlineList {
    fn note_down_inline(self, state: &mut ReadState) -> Vec<ParagraphItem>;
}

pub trait GetTextRange {
    fn as_range(&self) -> TextRange;
}

// impl GetTextRange for Option<Position> {
//     fn as_range(&self) -> TextRange {
//         match self {
//             Some(s) => TextRange { head_offset: s.start.offset as u32, tail_offset: s.end.offset as u32 },
//             None => TextRange { head_offset: 0, tail_offset: 0 },
//         }
//     }
// }
