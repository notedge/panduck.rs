use super::*;

impl<G> IntoSycamore<G> for Delimiter
where
    G: GenericNode,
{
    fn into_sycamore(self, _: &SycamoreBuilder) -> G {
        match self {
            Self::HorizontalRule => GenericNode::element("hr"),
        }
    }
}

impl<G> IntoSycamore<G> for Header
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        let node = match self.level {
            1 => GenericNode::element("h1"),
            2 => GenericNode::element("h2"),
            3 => GenericNode::element("h3"),
            4 => GenericNode::element("h4"),
            5 => GenericNode::element("h5"),
            _ => GenericNode::element("h6"),
        };
        push_nodes(&node, self.children, ctx);
        return node;
    }
}

impl<G> IntoSycamore<G> for CodeNode
where
    G: GenericNode,
{
    #[cfg(feature = "local")]
    fn into_sycamore(self, _: &SycamoreBuilder) -> G {
        todo!()
    }
}

impl<G> IntoSycamore<G> for MathNode
where
    G: GenericNode,
{
    #[cfg(feature = "local")]
    fn into_sycamore(self, _: &SycamoreBuilder) -> G {
        use panduck_html::extension::katex::{katex_display, katex_inline};
        match self.get_format().to_ascii_lowercase().as_str() {
            "tex" | "latex" => {
                let g: G = GenericNode::marker();
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
                match g.first_child() {
                    Some(s) => s,
                    #[cfg(debug_assertions)]
                    None => panic!("Illegal HTML content"),
                    #[cfg(not(debug_assertions))]
                    None => GenericNode::marker(),
                }
            }
            #[cfg(debug_assertions)]
            _ => return error_inline(&format!("Unknown math renderer {}", self.get_format())),
            #[cfg(not(debug_assertions))]
            _ => return GenericNode::marker(),
        }
    }
}
