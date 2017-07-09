use super::*;
use markdown::mdast::{Image, Link};
use wasi_notedown::exports::notedown::core::syntax_tree::{ImageReference, LinkReference, TableRow};

impl NoteInline for Node {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        match self {
            Node::MdxJsxFlowElement(_) => {
                todo!()
            }
            Node::MdxjsEsm(_) => {
                todo!()
            }
            Node::Toml(_) => {
                todo!()
            }
            Node::Yaml(_) => {
                todo!()
            }
            Node::Break(_) => {
                todo!()
            }
            Node::InlineCode(style) => style.note_down_inline(state),
            Node::InlineMath(style) => style.note_down_inline(state),
            Node::Delete(style) => style.note_down_inline(state),
            Node::Emphasis(style) => style.note_down_inline(state),
            Node::MdxTextExpression(_) => {
                todo!()
            }
            Node::FootnoteReference(_) => {
                todo!()
            }
            Node::Html(_) => {
                todo!()
            }
            Node::Link(link) => link.note_down_inline(state),
            Node::LinkReference(link) => link.note_down_inline(state),
            Node::Image(image) => image.note_down_inline(state),
            Node::ImageReference(image) => image.note_down_inline(state),
            Node::MdxJsxTextElement(_) => {
                todo!()
            }
            Node::Strong(v) => v.note_down_inline(state),
            Node::Text(v) => v.note_down_inline(state),
            Node::MdxFlowExpression(_) => {
                todo!()
            }
            Node::ThematicBreak(_) => {
                todo!()
            }
            _ => unreachable!()
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
        let image = ImageReference { url: Some(self.url), alternative: self.alt, range: self.position.as_range() };
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
        let image =
            LinkReference { url: Some(self.url), title: self.title.unwrap_or_default(), range: self.position.as_range() };
        Ok(ParagraphItem::Link(image))
    }
}

impl NoteInline for markdown::mdast::LinkReference {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let image = LinkReference { url: None, title: "".to_string(), range: self.position.as_range() };
        Ok(ParagraphItem::Link(image))
    }
}
