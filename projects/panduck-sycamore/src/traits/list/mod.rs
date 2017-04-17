use super::*;
use notedown_ast::nodes::{ListDetailedNode, ListSimpleNode};

impl<G> IntoSycamore<G> for ListView
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        match self {
            ListView::Quote(v) => v.into_sycamore(ctx),
            ListView::Ordered(v) => v.into_sycamore(ctx),
            ListView::Orderless(v) => v.into_sycamore(ctx),
            ListView::Details(v) => v.into_sycamore(ctx),
        }
    }
}
impl<G> IntoSycamore<G> for ListSimpleNode
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        todo!()
    }
}

impl<G> IntoSycamore<G> for ListDetailedNode
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        todo!()
    }
}
