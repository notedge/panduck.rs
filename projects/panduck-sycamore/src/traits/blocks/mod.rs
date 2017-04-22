use super::*;

mod math;

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
