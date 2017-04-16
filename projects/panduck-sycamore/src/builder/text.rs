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
            Self::Emoji(_) => {
                todo!()
            }
            TextNode::SoftNewline => {
                unimplemented!()
            }
            TextNode::HardNewline => {
                unimplemented!()
            }
            TextNode::CheckBox(_) => {
                unimplemented!()
            }
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
            Self::Emphasis => GenericNode::element("em"),
            Self::Underline => GenericNode::element("u"),
            Self::Undercover => GenericNode::element("u"),
            Self::Strong => GenericNode::element("strong"),
            Self::Delete => GenericNode::element("del"),
            Self::Insert => GenericNode::element("ins"),
            Self::ItalicBold => {
                unimplemented!()
            }
            Self::Marking => {
                unimplemented!()
            }
            Self::Color(_, _, _, _) => {
                unimplemented!()
            }
            Self::Subscript => GenericNode::element("sub"),
            Self::Superscript => GenericNode::element("sup"),
        }
    }
}
