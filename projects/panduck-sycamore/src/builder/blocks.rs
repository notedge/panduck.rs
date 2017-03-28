use super::*;

impl<G> IntoSycamore<G> for Delimiter
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        match self {
            Self::HorizontalRule => GenericNode::element("hr"),
        }
    }
}

impl<G> IntoSycamore<G> for Header
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        let node = match self.level {
            1 => GenericNode::element("h1"),
            2 => GenericNode::element("h2"),
            3 => GenericNode::element("h3"),
            4 => GenericNode::element("h4"),
            5 => GenericNode::element("h5"),
            _ => GenericNode::element("h6"),
        };
        push_nodes(&node, self.children);
        return node;
    }
}
