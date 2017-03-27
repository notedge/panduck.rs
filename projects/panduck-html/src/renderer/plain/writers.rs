use super::*;

impl PlainHTML for MathNode {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
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

impl PlainHTML for CodeNode {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        if let Some(renderer) = f.code_renderer {
            return f.write_str(&renderer(self));
        }
        unimplemented!()
    }
}

impl PlainHTML for Header {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        write!(f, "<h{0}>", self.level)?;
        self.children.plain_html(f)?;
        write!(f, "</h{0}>", self.level)?;
        Ok(())
    }
}

impl PlainHTML for ListView {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}
