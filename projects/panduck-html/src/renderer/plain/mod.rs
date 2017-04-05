use super::*;
use notedown_ast::nodes::{Delimiter, Literal, SmartLink, StyleNode};

mod table_view;
mod text;
mod writers;

pub trait PlainHTML {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result;
}

impl PlainHTML for ASTNodes {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        for child in self {
            child.plain_html(f)?
        }
        Ok(())
    }
}

impl<T: PlainHTML> PlainHTML for Literal<T> {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        self.value.plain_html(f)
    }
}

// notice that html5 is compatible with xhtml, but not the other way around
// so please close self-closing tags manually
// eg: <hr> -> <hr/>
impl PlainHTML for ASTKind {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        match self {
            ASTKind::Statements(children) => {
                f.write_str("<div>")?;
                children.plain_html(f);
                f.write_str("</div>")
            }
            ASTKind::Header(inner) => inner.plain_html(f),
            ASTKind::Paragraph(children) => {
                f.write_str("<p>")?;
                children.plain_html(f);
                f.write_str("</p>")
            }
            ASTKind::Delimiter(inner) => inner.plain_html(f),
            ASTKind::TableView(inner) => inner.plain_html(f),
            ASTKind::ListView(inner) => inner.plain_html(f),
            ASTKind::MathNode(inner) => inner.plain_html(f),
            ASTKind::CodeNode(inner) => inner.plain_html(f),
            ASTKind::LinkNode(inner) => inner.plain_html(f),
            ASTKind::TextSpan(inner) => inner.plain_html(f),
            ASTKind::StyledSpan(inner) => inner.plain_html(f),
            ASTKind::Command(_) => unimplemented!(),
            ASTKind::Value(_) => unimplemented!(),
        }
    }
}

impl PlainHTML for Delimiter {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        match self {
            Delimiter::HorizontalRule => match f.xhtml {
                true => f.write_str("<hr>"),
                false => f.write_str("<hr/>"),
            },
        }
    }
}

impl PlainHTML for SmartLink {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}
