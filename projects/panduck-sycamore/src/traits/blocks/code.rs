use super::*;

impl<G> IntoSycamore<G> for CodeNode
where
    G: GenericNode,
{
    #[cfg(feature = "local")]
    fn into_sycamore(self, builder: &SycamoreBuilder) -> G {
        let g: G = GenericNode::element("Fragment");
        let text = builder.config.code_config.syntect_config.render_html(&self);
        match text {
            Ok(o) => g.dangerously_set_inner_html(&o),
            #[cfg(debug_assertions)]
            Err(e) => return error_inline(e.to_string().as_str()),
            #[cfg(not(debug_assertions))]
            Err(_) => return GenericNode::marker(),
        }
        unwrap_inner(g)
    }
}
