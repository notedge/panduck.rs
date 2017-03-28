mod blocks;
mod text;
mod command;
mod table;
mod list;

use notedown_ast::nodes::ListView;
use notedown_ast::nodes::TableView;
use notedown_ast::{
    nodes::{Delimiter, Header, Literal},
    ASTKind, ASTNodes,
};
use sycamore::{generic_node::GenericNode};
use notedown_ast::nodes::{StyleKind, StyleNode, TextNode};

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
            ASTKind::Statements(children) => {
                let root: G = GenericNode::element("div");
                root.set_class_name("notedown");
                push_nodes(&root, children);
                return root;
            }
            ASTKind::Paragraph(children) => {
                let p = GenericNode::element("p");
                push_nodes(&p, children);
                return p;
            }
            ASTKind::Header(inner) => inner.into_sycamore(),
            ASTKind::Delimiter(inner) => inner.into_sycamore(),
            ASTKind::TableView(inner) => inner.into_sycamore(),
            ASTKind::ListView(inner) => inner.into_sycamore(),
            ASTKind::CodeNode(_) => {
                unimplemented!()
            }
            ASTKind::MathNode(_) => {
                unimplemented!()
            }
            ASTKind::LinkNode(_) => {
                unimplemented!()
            }
            ASTKind::TextSpan(inner) => inner.into_sycamore(),
            ASTKind::StyledSpan(inner) => inner.into_sycamore(),
            ASTKind::Command(_) => {
                unimplemented!()
            }
            ASTKind::Value(_) => {
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
