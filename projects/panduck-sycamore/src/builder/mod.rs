mod blocks;
mod command;
mod link;
mod list;
mod table;
mod text;

use notedown_ast::{
    nodes::{Delimiter, Header, ListView, Literal, StyleKind, StyleNode, TableView, TextNode},
    ASTKind, ASTNodes,
};
use sycamore::generic_node::GenericNode;

pub trait IntoSycamore<G: GenericNode> {
    fn into_sycamore(self) -> G;
}

impl<T, G> IntoSycamore<G> for Literal<T>
where
    T: IntoSycamore<G>,
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        self.value.into_sycamore()
    }
}

impl<G> IntoSycamore<G> for ASTKind
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        match self {
            Self::Statements(children) => {
                let root: G = GenericNode::element("div");
                root.set_class_name("notedown");
                push_nodes(&root, children);
                return root;
            }
            Self::Paragraph(children) => {
                let p = GenericNode::element("p");
                push_nodes(&p, children);
                return p;
            }
            Self::Header(inner) => inner.into_sycamore(),
            Self::Delimiter(inner) => inner.into_sycamore(),
            Self::TableView(inner) => inner.into_sycamore(),
            Self::ListView(inner) => inner.into_sycamore(),
            Self::CodeNode(inner) => inner.into_sycamore(),
            Self::MathNode(inner) => inner.into_sycamore(),
            Self::LinkNode(inner) => inner.into_sycamore(),
            Self::TextSpan(inner) => inner.into_sycamore(),
            Self::StyledSpan(inner) => inner.into_sycamore(),
            Self::Command(_) => {
                unimplemented!()
            }
            Self::Value(_) => {
                unimplemented!()
            }
        }
    }
}

pub fn push_nodes<G>(node: &G, children: ASTNodes)
where
    G: GenericNode,
{
    for i in children {
        node.append_child(&i.value.into_sycamore())
    }
}
