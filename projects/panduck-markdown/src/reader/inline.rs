use super::*;

impl NoteInline for Node {
    fn note_down_inline(self, state: &mut ReadState) -> Result<ParagraphItem, NotedownError> {
        match self {
            Node::Root(_) => {
                todo!()
            }
            Node::BlockQuote(_) => {
                todo!()
            }
            Node::FootnoteDefinition(_) => {
                todo!()
            }
            Node::MdxJsxFlowElement(_) => {
                todo!()
            }
            Node::List(_) => {
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
            Node::InlineCode(v) => v.note_down_inline(state),
            Node::InlineMath(v) => v.note_down_inline(state),
            Node::Delete(v) => v.note_down_inline(state),
            Node::Emphasis(v) => v.note_down_inline(state),
            Node::MdxTextExpression(_) => {
                todo!()
            }
            Node::FootnoteReference(_) => {
                todo!()
            }
            Node::Html(_) => {
                todo!()
            }
            Node::Image(_) => {
                todo!()
            }
            Node::ImageReference(_) => {
                todo!()
            }
            Node::MdxJsxTextElement(_) => {
                todo!()
            }
            Node::Link(_) => {
                todo!()
            }
            Node::LinkReference(_) => {
                todo!()
            }
            Node::Strong(v) => v.note_down_inline(state),
            Node::Text(v) => v.note_down_inline(state),
            Node::Code(_) => {
                todo!()
            }
            Node::Math(_) => {
                todo!()
            }
            Node::MdxFlowExpression(_) => {
                todo!()
            }
            Node::Heading(_) => {
                todo!()
            }
            Node::Table(_) => {
                todo!()
            }
            Node::ThematicBreak(_) => {
                todo!()
            }
            Node::TableRow(_) => {
                todo!()
            }
            Node::TableCell(_) => {
                todo!()
            }
            Node::ListItem(_) => {
                todo!()
            }
            Node::Definition(_) => {
                todo!()
            }
            Node::Paragraph(v) => {
                todo!()
            }
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

fn paragraph_items(children: Vec<Node>, state: &mut ReadState) -> Result<Vec<ParagraphItem>, NotedownError> {
    let mut items = Vec::with_capacity(children.len());
    for x in children {
        match x.note_down_inline(state) {
            Ok(o) => items.push(o),
            Err(e) => {
                state.errors.push(e);
            }
        }
    }
    Ok(items)
}
