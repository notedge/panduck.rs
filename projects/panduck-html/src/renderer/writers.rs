use super::*;

impl WriteHTML for ASTNodes {
    fn write_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        for child in self {
            child.write_html(f)?
        }
        Ok(())
    }
}

impl WriteHTML for ASTNode {
    fn write_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        self.value.write_html(f)
    }
}

// notice that html5 is compatible with xhtml, but not the other way around
// so please close self-closing tags manually
// eg: <hr> -> <hr/>
impl WriteHTML for ASTKind {
    fn write_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        match self {
            ASTKind::Statements(_) => unimplemented!(),
            ASTKind::Header(_) => unimplemented!(),
            ASTKind::Paragraph(children) => {
                f.write_str("<p>")?;
                children.write_html(f);
                f.write_str("</p>")
            }
            ASTKind::CodeNode(inner) => inner.write_html(f),
            ASTKind::TableView(inner) => inner.write_html(f),
            ASTKind::ListView(inner) => inner.write_html(f),
            ASTKind::CodeNode(inner) => unimplemented!(),
            ASTKind::TextSpan(inner) => inner.write_html(f),
            ASTKind::MathNode(_) => unimplemented!(),
            ASTKind::LinkNode(_) => unimplemented!(),
            ASTKind::Value(_) => unimplemented!(),
            ASTKind::Command(_) => unimplemented!(),
            ASTKind::Delimiter(_) => {}
            ASTKind::StyledSpan(_) => {}
        }
    }
}

impl WriteHTML for TextNode {
    fn write_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        match self.kind {
            StyleKind::Plain => {
                self.children.write_html(f)?;
            }
            StyleKind::Italic => {
                f.write_str("<i>")?;
                self.children.write_html(f)?;
                f.write_str("</i>")?;
            }
            StyleKind::Bold => {
                f.write_str("<b>")?;
                self.children.write_html(f)?;
                f.write_str("</b>")?;
            }
            StyleKind::Emphasis => {
                f.write_str("<em>")?;
                self.children.write_html(f)?;
                f.write_str("</em>")?;
            }
            StyleKind::Underline => {
                f.write_str("<u>")?;
                self.children.write_html(f)?;
                f.write_str("</u>")?;
            }
            StyleKind::Strikethrough => {
                f.write_str("<del>")?;
                self.children.write_html(f)?;
                f.write_str("</del>")?;
            }
            StyleKind::Undercover => {
                f.write_str(r#"<span class="undercover">"#)?;
                self.children.write_html(f)?;
                f.write_str("</span>")?;
            }
        }
        Ok(())
    }
}

impl WriteHTML for MathNode {
    fn write_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        if let Some(renderer) = f.math_renderer {
            return f.write_str(&renderer(self));
        }
        match self.get_kind() {
            MathKind::Inline => write!(f, r#"<span class="math">${}$</span>"#, self.get_text()),
            MathKind::Display => write!(f, r#"<span class="math">$\displaystyle{{{}}}$</span>"#, self.get_text()),
            MathKind::BlockInline => write!(f, r#"<div class="math">${}$</div>"#, self.get_text()),
            MathKind::BlockDisplay => write!(f, r#"<div class="math">$\displaystyle{{{}}}$</div>"#, self.get_text()),
        }
    }
}

impl WriteHTML for CodeNode {
    fn write_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        if let Some(renderer) = f.code_renderer {
            return f.write_str(&renderer(self));
        }
        unimplemented!()
    }
}

impl WriteHTML for Header {
    fn write_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        write!(f, "<h{0}>", self.level)?;
        self.children.write_html(f)?;
        write!(f, "</h{0}>", self.level)?;
        Ok(())
    }
}

impl WriteHTML for ListView {
    fn write_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}
