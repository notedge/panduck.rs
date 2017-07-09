use markdown::mdast::{Definition, FootnoteDefinition, Heading};
use wasi_notedown::exports::notedown::core::syntax_tree::HeadingBlock;
use super::*;

impl NoteBlock for Node {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        match self {
            Node::Heading(head) => head.note_down_block(state),
            Node::BlockQuote(quote) => quote.note_down_block(state),

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
            Node::MdxJsxTextElement(_) => {
                todo!()
            }
            Node::Code(code) => code.note_down_block(state),
            Node::MdxFlowExpression(_) => {
                todo!()
            }
            Node::ThematicBreak(_) => {
                todo!()
            }
            Node::Table(table) => table.note_down_block(state),
            Node::Definition(def) => def.note_down_block(state),
            Node::FootnoteDefinition(def) => def.note_down_block(state),
            Node::Paragraph(p) => p.note_down_block(state),
            _ => unreachable!(),
        }
    }
}

impl NoteBlock for Heading {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let title = paragraph_items(self.children, state)?;
        
        let head = HeadingBlock {
            level: self.depth,
            title: ParagraphBlock { terms: title, range: TextRange { head_offset: 0, tail_offset: 0 } },
            range: self.position.as_range(),
        };
        Ok(RootItem::Heading(head))
    }
}

impl NoteBlock for Paragraph {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let terms = paragraph_items(self.children, state)?;
        Ok(RootItem::Paragraph(ParagraphBlock { terms, range: self.position.as_range() }))
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

impl NoteBlock for Definition {
    fn note_down_block(self, _: &mut ReadState) -> Result<RootItem, NotedownError> {
        let content = MathContent::Tex("".to_string());
        let math =
            MathEnvironment { display: MathDisplay::Block, content, range: TextRange { head_offset: 0, tail_offset: 0 } };
        Ok(RootItem::Math(math))
    }
}


impl NoteBlock for FootnoteDefinition {
    fn note_down_block(self, _: &mut ReadState) -> Result<RootItem, NotedownError> {
        let content = MathContent::Tex("".to_string());
        let math =
            MathEnvironment { display: MathDisplay::Block, content, range: TextRange { head_offset: 0, tail_offset: 0 } };
        Ok(RootItem::Math(math))
    }
}
