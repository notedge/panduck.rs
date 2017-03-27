use super::*;

impl PlainHTML for TextNode {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        let raw = match self {
            TextNode::Normal(v) => v.to_owned(),
            TextNode::Raw(v) => v.to_owned(),
            TextNode::Escaped(v) => v.to_string(),
        };

        html_escape::encode_text(raw)
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
