use super::*;


impl PlainHTML for TextNode {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        match self {
            TextNode::Normal(v) => { f.write_str(v) }
            TextNode::Raw(v) => { f.write_str(v) }
            TextNode::Escaped(v) => { f.write_char(*v) }
        }
    }
}

impl PlainHTML for StyleNode {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        match self.kind {
            StyleKind::Plain => {
                self.children.plain_html(f)?;
            }
            StyleKind::Italic => {
                f.write_str("<i>")?;
                self.children.plain_html(f)?;
                f.write_str("</i>")?;
            }
            StyleKind::Bold => {
                f.write_str("<b>")?;
                self.children.plain_html(f)?;
                f.write_str("</b>")?;
            }
            StyleKind::Emphasis => {
                f.write_str("<em>")?;
                self.children.plain_html(f)?;
                f.write_str("</em>")?;
            }
            StyleKind::Underline => {
                f.write_str("<u>")?;
                self.children.plain_html(f)?;
                f.write_str("</u>")?;
            }
            StyleKind::Strikethrough => {
                f.write_str("<del>")?;
                self.children.plain_html(f)?;
                f.write_str("</del>")?;
            }
            StyleKind::Undercover => {
                f.write_str(r#"<span class="undercover">"#)?;
                self.children.plain_html(f)?;
                f.write_str("</span>")?;
            }
        }
        Ok(())
    }
}