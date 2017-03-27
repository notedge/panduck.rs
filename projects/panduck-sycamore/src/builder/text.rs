use super::*;
use notedown_ast::nodes::{StyleKind, StyleNode, TextNode};

// fn new<G:GenericNode>() -> Template<G> {
//     template! {
//     p {
//     span { "Hello " }
//     strong { "World!" }
//     }
//     }
// }

impl<G> IntoSycamore<G> for TextNode
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G
    where
        G: GenericNode,
    {
        match self {
            TextNode::Normal(s) => GenericNode::text_node(&s),
            TextNode::Raw(s) => GenericNode::text_node(&s),
            TextNode::Escaped(c) => GenericNode::text_node(c.to_string().as_str()),
        }
    }
}

impl<G> IntoSycamore<G> for StyleNode
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        let node: G = match self.kind {
            StyleKind::Plain => GenericNode::element("span"),
            StyleKind::Italic => GenericNode::element("i"),
            StyleKind::Bold => GenericNode::element("bold"),
            StyleKind::Emphasis => {
                unimplemented!()
            }
            StyleKind::Underline => GenericNode::element("u"),
            StyleKind::Strikethrough => {
                unimplemented!()
            }
            StyleKind::Undercover => {
                unimplemented!()
            }
        };
        for i in self.children {
            node.append_child(&i.value.into_sycamore())
        }
        return node;
    }
}

