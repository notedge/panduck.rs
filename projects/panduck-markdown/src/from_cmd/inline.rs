use super::*;
use markdown::mdast::{FootnoteReference, Image, Link, MdxTextExpression};
use std::str::FromStr;
use wasi_notedown::{
    exports::notedown::core::{
        syntax_tree::{ImageReference, LinkReference},
        types::Url,
    },
    UrlNative,
};

impl NoteInline for Node {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        match self {
            Node::InlineCode(style) => style.note_down_inline(state),
            Node::InlineMath(style) => style.note_down_inline(state),
            Node::Delete(style) => style.note_down_inline(state),
            Node::Emphasis(style) => style.note_down_inline(state),
            Node::Link(link) => link.note_down_inline(state),
            Node::LinkReference(link) => link.note_down_inline(state),
            Node::FootnoteReference(link) => link.note_down_inline(state),
            Node::Image(image) => image.note_down_inline(state),
            Node::ImageReference(image) => image.note_down_inline(state),
            Node::Strong(v) => v.note_down_inline(state),
            Node::Text(v) => v.note_down_inline(state),
            Node::MdxJsxFlowElement(_) => {
                todo!()
            }
            Node::MdxjsEsm(_) => {
                todo!()
            }
            Node::Break(_) => {
                todo!()
            }
            Node::MdxTextExpression(slot) => slot.note_down_inline(state),
            Node::Html(_) => {
                todo!()
            }
            Node::MdxJsxTextElement(_) => {
                todo!()
            }
            Node::MdxFlowExpression(_) => {
                todo!()
            }
            Node::ThematicBreak(_) => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
}
impl NoteInline for Text {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let text = NormalText { text: self.value, range: self.position.as_range() };

        Ok(ParagraphItem::Text(text))
    }
}

impl NoteInline for Strong {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        // let mut blocks = paragraph_items(self.children, state)?;
        let text = StyledText { type_: StyleType::BOLD, range: self.position.as_range() };
        Ok(ParagraphItem::Style(text))
    }
}

impl NoteInline for Delete {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        // let mut blocks = paragraph_items(self.children, state)?;
        let text = StyledText { type_: StyleType::ITALIC, range: self.position.as_range() };
        Ok(ParagraphItem::Style(text))
    }
}

impl NoteInline for Emphasis {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        // let mut blocks = paragraph_items(self.children, state)?;
        let text = StyledText { type_: StyleType::ITALIC, range: self.position.as_range() };
        Ok(ParagraphItem::Style(text))
    }
}

impl NoteInline for InlineCode {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let text = CodeEnvironment { action: CodeAction::Anonymous, lines: self.value, range: self.position.as_range() };
        Ok(ParagraphItem::Code(text))
    }
}

impl NoteInline for InlineMath {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let content = MathContent::Tex(self.value);
        let text = MathEnvironment { display: MathDisplay::Inline, content, range: self.position.as_range() };
        Ok(ParagraphItem::Math(text))
    }
}

impl NoteInline for Image {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let url = Url::new(UrlNative::from_str(&self.url)?);
        let image = ImageReference { url: Some(url), alternative: self.alt, range: self.position.as_range() };
        Ok(ParagraphItem::Image(image))
    }
}

impl NoteInline for markdown::mdast::ImageReference {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let image = ImageReference { url: None, alternative: "".to_string(), range: self.position.as_range() };
        Ok(ParagraphItem::Image(image))
    }
}
impl NoteInline for Link {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let url = Url::new(UrlNative::from_str(&self.url)?);
        let image = LinkReference { url: Some(url), title: self.title.unwrap_or_default(), range: self.position.as_range() };
        Ok(ParagraphItem::Link(image))
    }
}

impl NoteInline for markdown::mdast::LinkReference {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let image = LinkReference { url: None, title: "".to_string(), range: self.position.as_range() };
        Ok(ParagraphItem::Link(image))
    }
}
impl NoteInline for FootnoteReference {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let image = LinkReference { url: None, title: "".to_string(), range: self.position.as_range() };
        Ok(ParagraphItem::Link(image))
    }
}

impl NoteInline for MdxTextExpression {
    /// \js("AAA")
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let image = LinkReference { url: None, title: "".to_string(), range: self.position.as_range() };
        Ok(ParagraphItem::Link(image))
    }
}
