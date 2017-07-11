use crate::utils::{NoteBlock, NoteInline, NoteInlineList, ReadState};
use pandoc_ast::{Block, Inline, MathType};
use std::str::FromStr;
use wasi_notedown::{
    exports::notedown::core::{
        syntax_tree::{
            HeadingBlock, ImageReference, LinkReference, ParagraphBlock, ParagraphItem, RootItem, StyleType, StyledText,
        },
        types::{NotedownError, TextRange, Url},
    },
    UrlNative,
};

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
            Self::CodeBlock(attr, code) => unimplemented!(),
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
                    title: ParagraphBlock {
                        terms: children.note_down_inline(state),
                        range: TextRange { head_offset: 0, tail_offset: 0 },
                    },
                    range: TextRange { head_offset: 0, tail_offset: 0 },
                };
                RootItem::Heading(heading)
            }
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
    fn note_down_inline(self, state: &mut ReadState) -> Vec<ParagraphItem> {
        let mut list = Vec::with_capacity(self.len());
        for item in self {
            match item.note_down_inline(state) {
                Ok(o) => list.push(o),
                Err(e) => state.note_error(e),
            }
        }
        list
    }
}

impl NoteInline for Inline {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let item = match self {
            Self::Str(s) => unimplemented!(),
            Self::Emph(v) => {
                let items = v.note_down_inline(state);
                let style = StyledText { type_: StyleType::ITALIC, range: TextRange { head_offset: 0, tail_offset: 0 } };
                ParagraphItem::Style(style)
            }
            Self::Underline(v) => {
                let items = v.note_down_inline(state);
                let style = StyledText { type_: StyleType::UNDERLINE, range: TextRange { head_offset: 0, tail_offset: 0 } };
                ParagraphItem::Style(style)
            }
            Self::Strong(v) => {
                let items = v.note_down_inline(state);
                let style = StyledText { type_: StyleType::BOLD, range: TextRange { head_offset: 0, tail_offset: 0 } };
                ParagraphItem::Style(style)
            }
            Self::Strikeout(v) => {
                let items = v.note_down_inline(state);
                let style = StyledText { type_: StyleType::STRIKETHROUGH, range: TextRange { head_offset: 0, tail_offset: 0 } };
                ParagraphItem::Style(style)
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
            Self::Code(attr, code) => unimplemented!(),
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
            Self::Link(attr, text, (url, title)) => {
                let url = Url::new(UrlNative::from_str(&url)?);

                let link = LinkReference { url: Some(url), title, range: TextRange { head_offset: 0, tail_offset: 0 } };
                ParagraphItem::Link(link)
            }
            Self::Image(attr, text, (url, title)) => {
                let url = Url::new(UrlNative::from_str(&url)?);

                let link =
                    ImageReference { url: Some(url), range: TextRange { head_offset: 0, tail_offset: 0 }, alternative: title };
                ParagraphItem::Image(link)
            }
            Self::Note(_) => {
                unimplemented!()
            }
            Self::Span(_, _) => {
                unimplemented!()
            }
        };
        Ok(item)
    }
}

fn make_styled(children: Vec<Inline>, state: &mut ReadState) -> ParagraphItem {
    let items = children.note_down_inline(state);
    let style = StyledText { type_: StyleType::UNDERLINE, range: TextRange { head_offset: 0, tail_offset: 0 } };
    ParagraphItem::Style(style)
}