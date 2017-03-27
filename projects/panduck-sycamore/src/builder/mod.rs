mod text;

use notedown_ast::{nodes::Literal, ASTKind, ASTNodes};
use sycamore::{generic_node::GenericNode, template, template::Template};

pub trait IntoSycamore<G: GenericNode> {
    fn into_sycamore(self) -> G;
}

impl<G> IntoSycamore<G> for ASTNodes
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        todo!()
    }
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
        todo!()
    }
}
