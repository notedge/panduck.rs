
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
pub trait NoteInlineList {
    fn note_down_inline(self, state: &mut ReadState) -> Vec<ParagraphItem>;
}

pub trait GetTextRange {
    fn as_range(&self) -> TextRange;
}

impl GetTextRange for Option<Position> {
    fn as_range(&self) -> TextRange {
        match self {
            Some(s) => TextRange { head_offset: s.start.offset as u32, tail_offset: s.end.offset as u32 },
            None => TextRange { head_offset: 0, tail_offset: 0 },
        }
    }
}

pub fn root_items(children: Vec<Node>, state: &mut ReadState) -> Result<Vec<RootItem>, NotedownError> {
    let mut blocks = Vec::with_capacity(children.len());
    for x in children {
        match x.note_down_block(state) {
            Ok(o) => blocks.push(o),
            Err(e) => {
                state.errors.push(e);
            }
        }
    }
    Ok(blocks)
}

