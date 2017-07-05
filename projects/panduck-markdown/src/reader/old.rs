use markdown::mdast::{Delete, Emphasis, InlineCode, Strong};
use wasi_notedown::exports::notedown::core::syntax_tree::{CodeAction,  CodeSpan, NormalText, StyledText, StyleType};
use crate::utils::GetTextRange;
use super::*;


impl NoteInline for Text {
    fn note_down_inline(self, _: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let text = NormalText { text: self.value, range: self.position.as_range() };

        Ok(ParagraphItem::Text(text))
    }
}

impl NoteInline for Strong {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        // let mut blocks = paragraph_items(self.children, state)?;
        let text = StyledText {
            type_: StyleType::BOLD,
            range: self.position.as_range(),
        };
        Ok(ParagraphItem::Style(text))
    }
}

impl NoteInline for Delete {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        // let mut blocks = paragraph_items(self.children, state)?;
        let text = StyledText {
            type_: StyleType::ITALIC,
            range: self.position.as_range(),
        };
        Ok(ParagraphItem::Style(text))
    }
}


impl NoteInline for Emphasis {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        // let mut blocks = paragraph_items(self.children, state)?;
        let text = StyledText {
            type_: StyleType::ITALIC,
            range: self.position.as_range(),
        };
        Ok(ParagraphItem::Style(text))
    }
}


impl NoteInline for InlineCode {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        let text = CodeSpan {
            action: CodeAction::Inine,
            lines: self.value,
            range: self.position.as_range(),
        };
        Ok(ParagraphItem::Code(text))
    }
}


fn paragraph_items(children: Vec<Node>, state: &mut ReadState) -> Result<Vec<ParagraphItem>, NotedownError> {
    let mut items = Vec::with_capacity(children.len());
    for x in children {
        match x.note_down_inline(state) {
            Ok(o) => { items.push(o) }
            Err(e) => {
                state.errors.push(e);
            }
        }
    }
    Ok(items)
}
