use super::*;
use notedown_ast::nodes::{EmailLink, HyperLink, ImageLink, SmartLink, TagReference, TwoWayLink};

impl<G> IntoSycamore<G> for SmartLink
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        match self {
            Self::EMail(v) => v.into_sycamore(),
            Self::Normal(v) => v.into_sycamore(),
            Self::Image(v) => v.into_sycamore(),
            Self::TwoWay(v) => v.into_sycamore(),
            Self::Reference(v) => v.into_sycamore(),
        }
    }
}

impl<G> IntoSycamore<G> for EmailLink
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        todo!()
    }
}
impl<G> IntoSycamore<G> for HyperLink
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        todo!()
    }
}
impl<G> IntoSycamore<G> for ImageLink
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        todo!()
    }
}
impl<G> IntoSycamore<G> for TwoWayLink
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        todo!()
    }
}
impl<G> IntoSycamore<G> for TagReference
where
    G: GenericNode,
{
    fn into_sycamore(self) -> G {
        todo!()
    }
}
