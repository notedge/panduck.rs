use super::*;

impl<G> IntoSycamore<G> for CodeNode
where
    G: GenericNode,
{
    #[cfg(feature = "local")]
    fn into_sycamore(self, _: &SycamoreBuilder) -> G {
        let pre: G = GenericNode::element("pre");
        pre.update_inner_text(&self.to_string());
        return pre;
    }
}
