use crate::utils::{NoteBlock, NoteBlockList, NoteInline, NoteInlineList, NoteRoot, ReadState};
use orgize::{
    ast::{Document, Headline, HeadlineTitle},
    rowan::ast::{AstChildren, AstNode},
    ParseConfig, SyntaxKind, SyntaxNode, SyntaxNodeChildren,
};
use wasi_notedown::exports::notedown::core::{
    syntax_tree::{NotedownRoot, ParagraphItem, RootItem},
    types::{NotedownError, Object},
};

// mod blocks;
// mod html;
// mod inline;
// mod list;
// mod table;

pub struct MarkdownParser {}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self {}
    }
}

impl MarkdownParser {
    pub fn load_str(&self, input: &str) -> Result<NotedownRoot, Vec<NotedownError>> {
        let config = ParseConfig { ..Default::default() };
        let mut state = ReadState::default();
        let org = config.parse(input);
        let root = org.document().note_down_root(&mut state);
        state.finish(root)
    }
}
impl NoteRoot for Document {
    fn note_down_root(self, state: &mut ReadState) -> Result<NotedownRoot, NotedownError> {
        let heads = self.headlines().note_down_block(state);
        Ok(NotedownRoot {
            blocks: heads,
            config: Object { map: vec![] },
            path: None,
        })
        
        
    }
}
impl NoteBlock for Headline {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        todo!()
    }
}

impl<N: AstNode + NoteBlock> NoteBlockList for AstChildren<N> {
    fn note_down_block(self, state: &mut ReadState) -> Vec<RootItem> {
        let mut out = Vec::with_capacity(self.size_hint().0);
        for node in self {
            match node.note_down_block(state) {
                Ok(o) => out.push(o),
                Err(e) => state.note_error(e),
            }
        }
        out
    }
}

impl<'i> NoteRoot for &'i SyntaxNode {
    fn note_down_root(self, state: &mut ReadState) -> Result<NotedownRoot, NotedownError> {
        match self.kind() {
            SyntaxKind::DOCUMENT => {
                let blocks = self.children().note_down_block(state);
                Ok(NotedownRoot { blocks, config: Object { map: vec![] }, path: None })
            }
            _ => unreachable!("SyntaxKind::{:?} => {{}}", self.kind()),
        }
    }
}

impl NoteBlockList for SyntaxNodeChildren {
    fn note_down_block(self, state: &mut ReadState) -> Vec<RootItem> {
        let mut out = Vec::with_capacity(self.size_hint().0);
        for node in self {
            match node.note_down_block(state) {
                Ok(o) => out.push(o),
                Err(e) => state.note_error(e),
            }
        }
        out
    }
}

impl NoteInlineList for SyntaxNodeChildren {
    fn note_down_inline(self, state: &mut ReadState) -> Vec<ParagraphItem> {
        let mut out = Vec::with_capacity(self.size_hint().0);
        for node in self {
            match node.note_down_inline(state) {
                Ok(o) => out.push(o),
                Err(e) => state.note_error(e),
            }
        }
        out
    }
}

impl NoteBlock for SyntaxNode {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        match self.kind() {
            SyntaxKind::SECTION => Ok(RootItem::Placeholder),
            SyntaxKind::HEADLINE => {
                let inline = self.children().note_down_inline(state);
                todo!()
            }
            _ => unreachable!("SyntaxKind::{:?} => {{}}", self.kind()),
        }
    }
}

impl NoteInline for SyntaxNode {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        match self.kind() {
            SyntaxKind::HEADLINE => Ok(ParagraphItem::Placeholder),
            SyntaxKind::HEADLINE_TITLE => Ok(ParagraphItem::Placeholder),
            _ => unreachable!("SyntaxKind::{:?} => {{}}", self.kind()),
        }
    }
}

#[test]
fn ready() {
    println!("it works!")
}
