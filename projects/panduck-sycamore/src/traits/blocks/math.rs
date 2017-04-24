use super::*;

impl<G> IntoSycamore<G> for MathNode
where
    G: GenericNode,
{
    #[cfg(feature = "local")]
    fn into_sycamore(self, _: &SycamoreBuilder) -> G {
        use panduck_html::extension::katex::{katex_display, katex_inline};
        match self.get_format().to_ascii_lowercase().as_str() {
            "tex" | "latex" => {
                let g: G = GenericNode::element("Fragment");
                let html = match self.get_kind() {
                    MathKind::Inline => katex_inline(&self.get_text()),
                    MathKind::Display => katex_display(&self.get_text()),
                    MathKind::BlockInline => katex_inline(&self.get_text()),
                    MathKind::BlockDisplay => katex_display(&self.get_text()),
                };
                match html {
                    Ok(o) => g.dangerously_set_inner_html(&o),
                    #[cfg(debug_assertions)]
                    Err(e) => return error_inline(e.to_string().as_str()),
                    #[cfg(not(debug_assertions))]
                    Err(_) => return GenericNode::marker(),
                };
                unwrap_inner(g)
            }
            #[cfg(debug_assertions)]
            _ => return error_inline(&format!("Unknown math renderer {}", self.get_format())),
            #[cfg(not(debug_assertions))]
            _ => return GenericNode::marker(),
        }
    }
}
