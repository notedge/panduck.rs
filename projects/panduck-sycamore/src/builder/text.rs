use super::*;


impl<G> IntoSycamore<G> for TextNode
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G
    where
        G: GenericNode,
    {
        match self {
            Self::Normal(s) => GenericNode::text_node(&s),
            Self::Raw(s) => GenericNode::text_node(&s),
            Self::Escaped(c) => GenericNode::text_node(c.to_string().as_str()),
        }
    }
}

impl<G> IntoSycamore<G> for StyleNode
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        let node: G = self.kind.into_sycamore();
        // for i in self.children {
        //     node.append_child(&i.value.into_sycamore())
        // }
        push_nodes(&node, self.children);
        return node;
    }
}

impl<G> IntoSycamore<G> for StyleKind
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        match self {
            Self::Plain => GenericNode::element("span"),
            Self::Italic => GenericNode::element("i"),
            Self::Bold => GenericNode::element("b"),
            Self::Emphasis => {
                unimplemented!()
            }
            Self::Underline => GenericNode::element("u"),
            Self::Strikethrough => {
                unimplemented!()
            }
            Self::Undercover => {
                unimplemented!()
            }
        }
    }
}
