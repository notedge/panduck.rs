use super::*;
use markdown::mdast::{Definition, FootnoteDefinition, Heading, Toml, Yaml};
use wasi_notedown::exports::notedown::core::syntax_tree::HeadingBlock;

impl NoteBlock for Node {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        match self {
            Node::Heading(head) => head.note_down_block(state),
            Node::BlockQuote(quote) => quote.note_down_block(state),
            Node::List(list) => list.note_down_block(state),
            Node::Toml(config) => config.note_down_block(state),
            Node::Yaml(config) => config.note_down_block(state),

            Node::Code(code) => code.note_down_block(state),

            Node::Table(table) => table.note_down_block(state),
            Node::Definition(def) => def.note_down_block(state),
            Node::FootnoteDefinition(def) => def.note_down_block(state),
            Node::Paragraph(p) => p.note_down_block(state),
            Node::Break(_) => {
                todo!()
            }
            Node::Html(_) => {
                todo!()
            }
            Node::ThematicBreak(_) => {
                todo!()
            }
            Node::MdxjsEsm(_) => {
                todo!()
            }
            Node::MdxTextExpression(_) => {
                todo!()
            }
            Node::MdxJsxFlowElement(_) => {
                todo!()
            }
            Node::MdxJsxTextElement(_) => {
                todo!()
            }
            Node::MdxFlowExpression(_) => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
}

impl NoteBlock for Toml {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        Ok(RootItem::Placeholder)
    }
}

impl NoteBlock for Yaml {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        Ok(RootItem::Placeholder)
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
