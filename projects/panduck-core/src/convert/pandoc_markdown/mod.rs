use crate::ToNotedown;
use notedown_ast::{ASTKind, ASTNode, ASTNodes};
use pandoc_ast::{Block, Inline, MathType};

impl ToNotedown for Vec<Block> {
    fn into_notedown(self) -> ASTNode {
        ASTKind::statements(self.into_notedown_list(), None)
    }

    fn into_notedown_list(self) -> ASTNodes {
        self.into_iter().map(|f| f.into_notedown()).collect()
    }
}

impl ToNotedown for Block {
    fn into_notedown(self) -> ASTNode {
        match self {
            Self::Plain(_) => {
                unimplemented!()
            }
            Self::Para(v) => ASTKind::paragraph(v.into_notedown_list(), None),
            Self::Div(_, children) => ASTKind::paragraph(children.into_notedown_list(), None),
            Self::LineBlock(_) => {
                unimplemented!()
            }
            Self::CodeBlock(_, code) => ASTKind::code_block(code, "text", None),
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
            Self::Header(level, _, children) => ASTKind::header(children.into_notedown_list(), level as u8, None),
            Self::HorizontalRule => ASTKind::hr(None),
            Self::Table(_, _, _, _, _, _) => {
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
        ASTKind::paragraph(self.into_notedown_list(), None)
    }

    fn into_notedown_list(self) -> ASTNodes {
        self.into_iter().map(|f| f.into_notedown()).collect()
    }
}

impl ToNotedown for Inline {
    fn into_notedown(self) -> ASTNode {
        match self {
            Inline::Str(s) => ASTKind::text(s, None),
            Inline::Emph(v) => ASTKind::emphasis(v.into_notedown_list(), None),
            Inline::Underline(v) => ASTKind::underline(v.into_notedown_list(), None),
            Inline::Strong(v) => ASTKind::strong(v.into_notedown_list(), None),
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
            Inline::Code(_, code) => ASTKind::code_inline(code, None),
            Inline::Space => ASTKind::text(" ", None),
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
