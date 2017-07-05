use super::*;

impl NoteBlock for Node {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        match self {
            Node::BlockQuote(quote) => quote.note_down_block(state),
            Node::FootnoteDefinition(_) => {
                todo!()
            }
            Node::MdxJsxFlowElement(_) => {
                todo!()
            }
            Node::List(list) => list.note_down_block(state),
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
            Node::InlineCode(_) => {
                todo!()
            }
            Node::InlineMath(_) => {
                todo!()
            }
            Node::Delete(_) => {
                todo!()
            }
            Node::Emphasis(_) => {
                todo!()
            }
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
            Node::Strong(_) => {
                todo!()
            }
            Node::Text(_) => {
                todo!()
            }
            Node::Code(code) => code.note_down_block(state),
            Node::Math(_) => {
                todo!()
            }
            Node::MdxFlowExpression(_) => {
                todo!()
            }
            Node::Heading(_) => {
                todo!()
            }

            Node::ThematicBreak(_) => {
                todo!()
            }
            Node::Table(table) => table.note_down_block(state),
            Node::TableRow(table) => table.note_down_block(state),
            Node::TableCell(table) => table.note_down_block(state),
            Node::ListItem(_) => {
                todo!()
            }
            Node::Definition(_) => {
                todo!()
            }
            Node::Paragraph(v) => v.note_down_block(state),
            _ => unreachable!(),
        }
    }
}
impl NoteBlock for Paragraph {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let mut blocks = Vec::with_capacity(self.children.len());
        for x in self.children {
            match x.note_down_inline(state) {
                Ok(o) => blocks.push(o),
                Err(e) => {
                    state.errors.push(e);
                }
            }
        }
        Ok(RootItem::Paragraph(ParagraphBlock { terms: vec![], range: TextRange { head_offset: 0, tail_offset: 0 } }))
    }
}

impl NoteBlock for Code {
    fn note_down_block(self, _: &mut ReadState) -> Result<RootItem, NotedownError> {
        let math = CodeEnvironment {
            action: CodeAction::Highlight(CodeHighlight { language: self.lang.unwrap(), range: self.position.as_range() }),
            lines: self.value,
            range: self.position.as_range(),
        };
        Ok(RootItem::Code(math))
    }
}

impl NoteBlock for Math {
    fn note_down_block(self, _: &mut ReadState) -> Result<RootItem, NotedownError> {
        let content = MathContent::Tex(self.value);
        let math =
            MathEnvironment { display: MathDisplay::Block, content, range: TextRange { head_offset: 0, tail_offset: 0 } };
        Ok(RootItem::Math(math))
    }
}
