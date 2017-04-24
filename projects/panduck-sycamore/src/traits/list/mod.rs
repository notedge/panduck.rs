use super::*;
use notedown_ast::nodes::{ListDetailedNode, ListItem, ListPrefixSymbol, ListSimpleNode};

impl<G> IntoSycamore<G> for ListView
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        match self {
            ListView::Quote(v) => {
                let item: G = GenericNode::element("blockquote");
                push_nodes(&item, v.children, ctx);
                return item;
            }
            ListView::Ordered(v) => {
                let item: G = GenericNode::element("ol");
                push_nodes(&item, v.children, ctx);
                return item;
            }
            ListView::Orderless(v) => {
                let item: G = GenericNode::element("ul");
                push_nodes(&item, v.children, ctx);
                return item;
            }
            ListView::Details(v) => v.into_sycamore(ctx),
        }
    }
}

impl<G> IntoSycamore<G> for ListItem
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        let item: G = GenericNode::element("li");
        push_nodes(&item, self.rest, ctx);
        return item;
    }
}

impl<G> IntoSycamore<G> for ListDetailedNode
where
    G: GenericNode,
{
    fn into_sycamore(self, _: &SycamoreBuilder) -> G {
        todo!()
    }
}
