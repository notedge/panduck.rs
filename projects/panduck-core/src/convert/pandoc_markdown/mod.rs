use crate::ToNotedown;
use notedown_ast::{ASTKind, ASTNode, ASTNodes};
use pandoc_ast::{Block, Inline, MathType};

impl ToNotedown for Block {
    fn into_notedown(self) -> ASTNode {
        match self {
            Self::Plain(v) => {
                unimplemented!()
            }
            Self::Para(v) => {
                unimplemented!()
            }
            Self::LineBlock(_) => {
                unimplemented!()
            }
            Self::CodeBlock(_, _) => {
                unimplemented!()
            }
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
            Self::Header(_, _, _) => {
                unimplemented!()
            }
            Self::HorizontalRule => ASTKind::hr(None),
            Self::Table(_, _, _, _, _, _) => {
                unimplemented!()
            }
            Self::Div(_, _) => {
                unimplemented!()
            }
            Self::Null => {
                unimplemented!()
            }
        }
    }
}

impl ToNotedown for Vec<Inline> {
    fn into_notedown(self) -> ASTNode {
        ASTKind::statements(self.into_notedown_list(), None)
    }

    fn into_notedown_list(self) -> ASTNodes {
        self.into_iter().map(|f| f.into_notedown()).collect()
    }
}

impl ToNotedown for Inline {
    fn into_notedown(self) -> ASTNode {
        match self {
            Inline::Str(_) => {
                unimplemented!()
            }
            Inline::Emph(_) => {
                unimplemented!()
            }
            Inline::Underline(v) => {
                unimplemented!()
            }
            Inline::Strong(v) => {
                unimplemented!()
            }
            Inline::Strikeout(_) => {
                unimplemented!()
            }
            Inline::Superscript(_) => {
                unimplemented!()
            }
            Inline::Subscript(_) => {
                unimplemented!()
            }
            Inline::SmallCaps(_) => {
                unimplemented!()
            }
            Inline::Quoted(_, _) => {
                unimplemented!()
            }
            Inline::Cite(_, _) => {
                unimplemented!()
            }
            Inline::Code(_, _) => {
                unimplemented!()
            }
            Inline::Space => {
                unimplemented!()
            }
            Inline::SoftBreak => ASTKind::soft_break(None),
            Inline::LineBreak => ASTKind::hard_break(None),
            Inline::Math(m, t) => match m {
                MathType::DisplayMath => {
                    unimplemented!()
                }
                MathType::InlineMath => {
                    unimplemented!()
                }
            },
            Inline::RawInline(_, _) => {
                unimplemented!()
            }
            Inline::Link(_, _, _) => {
                unimplemented!()
            }
            Inline::Image(_, _, _) => {
                unimplemented!()
            }
            Inline::Note(_) => {
                unimplemented!()
            }
            Inline::Span(_, _) => {
                unimplemented!()
            }
        }
    }
}
