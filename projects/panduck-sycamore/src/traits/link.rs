use super::*;
use notedown_ast::nodes::{EmailLink, HyperLink, ImageLink, SmartLink, TagReference, TwoWayLink};

impl<G> IntoSycamore<G> for SmartLink
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        match self {
            Self::EMail(v) => v.into_sycamore(ctx),
            Self::Normal(v) => v.into_sycamore(ctx),
            Self::Image(v) => v.into_sycamore(ctx),
            Self::TwoWay(v) => v.into_sycamore(ctx),
            Self::Reference(v) => v.into_sycamore(ctx),
        }
    }
}

impl<G> IntoSycamore<G> for EmailLink
where
    G: GenericNode,
{
    fn into_sycamore(self, _: &SycamoreBuilder) -> G {
        // <a href="mailto:someone@example.com">Send email</a>
        let a = GenericNode::element("a");
        return a;
    }
}
impl<G> IntoSycamore<G> for HyperLink
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        let a = GenericNode::element("a");
        return a;
    }
}
impl<G> IntoSycamore<G> for ImageLink
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        let img = GenericNode::element("img");
        return img;
    }
}
impl<G> IntoSycamore<G> for TwoWayLink
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        todo!()
    }
}
impl<G> IntoSycamore<G> for TagReference
where
    G: GenericNode,
{
    fn into_sycamore(self, ctx: &SycamoreBuilder) -> G {
        todo!()
    }
}
