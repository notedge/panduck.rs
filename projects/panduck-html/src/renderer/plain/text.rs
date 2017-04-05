use super::*;

impl PlainHTML for TextNode {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        let raw = match self {
            Self::Normal(v) => v.to_owned(),
            Self::Raw(v) => v.to_owned(),
            Self::Escaped(v) => v.to_string(),
            Self::Emoji(v) => v.to_string(),
        };

        f.write_str(&html_escape::encode_text(&raw).to_string())
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
            StyleKind::Undercover => {
                f.write_str(r#"<span class="undercover">"#)?;
                self.children.plain_html(f)?;
                f.write_str("</span>")?;
            }
            StyleKind::Strong => {
                f.write_str(r#"<strong>"#)?;
                self.children.plain_html(f)?;
                f.write_str("</strong>")?;
            }
            StyleKind::Highlight => {
                f.write_str(r#"<span class="undercover">"#)?;
                self.children.plain_html(f)?;
                f.write_str("</span>")?;
            }
            StyleKind::Delete => {
                f.write_str(r#"<del>"#)?;
                self.children.plain_html(f)?;
                f.write_str("</del>")?;
            }
            StyleKind::Insert => {
                f.write_str(r#"<ins>"#)?;
                self.children.plain_html(f)?;
                f.write_str("</ins>")?;
            }
        }
        Ok(())
    }
}
