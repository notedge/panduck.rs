use crate::utils::{NoteBlock, NoteInline, NoteInlineList, ReadState};
use pandoc_ast::{Block, Inline, MathType};
use wasi_notedown::exports::notedown::core::{
    syntax_tree::{ParagraphItem, RootItem},
    types::NotedownError,
};
use wasi_notedown::exports::notedown::core::syntax_tree::{HeadingBlock, ParagraphBlock};
use wasi_notedown::exports::notedown::core::types::TextRange;

impl NoteBlock for Vec<Block> {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        todo!()
    }
}

impl NoteBlock for Block {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let item = match self {
            Self::Plain(_) => {
                unimplemented!()
            }
            Self::Para(v) => unimplemented!(),
            Self::Div(_, children) => unimplemented!(),
            Self::LineBlock(_) => {
                unimplemented!()
            }
            Self::CodeBlock(_, code) => unimplemented!(),
            Self::RawBlock(_, _) => {
                unimplemented!()
            }
            Self::BlockQuote(_) => {
                unimplemented!()
            }
            Self::OrderedList(_, _) => {
                unimplemented!()
            }
            Self::BulletList(_) => {
                unimplemented!()
            }
            Self::DefinitionList(_) => {
                unimplemented!()
            }
            Self::Header(level, attr, children) => {

                let heading = HeadingBlock {
                    level: level as u8,
                    title: ParagraphBlock { terms: vec![], range: TextRange { head_offset: 0, tail_offset: 0 } },
                    range: TextRange { head_offset: 0, tail_offset: 0 } ,
                };
                RootItem::Heading(heading)
                
                
            },
            Self::HorizontalRule => unimplemented!(),
            Self::Table(_, _, _, _, _, _) => {
                unimplemented!()
            }
            Self::Figure(_, _, _) => {
                unimplemented!()
            }
            Self::Null => {
                unimplemented!()
            }
        };
        Ok(item)
    }
}

impl NoteInlineList for Vec<Inline> {
    fn note_down_inline(self, state: &mut ReadState) -> Result<Vec<ParagraphItem>, NotedownError> {
        todo!()
    }
}

impl NoteInline for Inline {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        match self {
            Self::Str(s) => unimplemented!(),
            Self::Emph(v) => unimplemented!(),
            Self::Underline(v) => unimplemented!(),
            Self::Strong(v) => unimplemented!(),
            Self::Strikeout(_) => {
                unimplemented!()
            }
            Self::Superscript(v) => unimplemented!(),
            Self::Subscript(v) => unimplemented!(),
            Self::SmallCaps(_) => {
                unimplemented!()
            }
            Self::Quoted(_, _) => {
                unimplemented!()
            }
            Self::Cite(_, _) => {
                unimplemented!()
            }
            Self::Code(_, code) => unimplemented!(),
            Self::Space => unimplemented!(),
            Self::SoftBreak => unimplemented!(),
            Self::LineBreak => unimplemented!(),
            Self::Math(m, t) => match m {
                MathType::DisplayMath => unimplemented!(),
                MathType::InlineMath => unimplemented!(),
            },
            Self::RawInline(_, _) => {
                unimplemented!()
            }
            Self::Link(_, _, _) => {
                unimplemented!()
            }
            Self::Image(_, _, _) => {
                unimplemented!()
            }
            Self::Note(_) => {
                unimplemented!()
            }
            Self::Span(_, _) => {
                unimplemented!()
            }
        }
    }
}
