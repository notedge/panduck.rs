use markdown::{mdast::Node, unist::Position};
use wasi_notedown::exports::notedown::core::{
    syntax_tree::{NotedownRoot, ParagraphItem, RootItem},
    types::{NotedownError, Object, TextRange, Url},
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
    pub fn get_object(&mut self) -> Object {
        Object { map: vec![] }
    }
    pub fn get_path(&self) -> Option<Url> {
        None
    }
}

pub trait NoteRoot {
    fn note_down_root(self, state: &mut ReadState) -> NotedownRoot;
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

impl GetTextRange for Option<Position> {
    fn as_range(&self) -> TextRange {
        match self {
            Some(s) => TextRange { head_offset: s.start.offset as u32, tail_offset: s.end.offset as u32 },
            None => TextRange { head_offset: 0, tail_offset: 0 },
        }
    }
}
pub fn group_block<T: NoteBlock>(list: Vec<T>, state: &mut ReadState) -> Vec<RootItem> {
    let mut blocks = Vec::with_capacity(list.len());
    for x in list {
        match x.note_down_block(state) {
            Ok(o) => blocks.push(o),
            Err(e) => {
                state.note_error(e);
            }
        }
    }
    blocks
}

pub fn group_inline<T: NoteInline>(list: Vec<T>, state: &mut ReadState) -> Vec<ParagraphItem> {
    let mut blocks = Vec::with_capacity(list.len());
    for x in list {
        match x.note_down_inline(state) {
            Ok(o) => blocks.push(o),
            Err(e) => {
                state.note_error(e);
            }
        }
    }
    blocks
}
