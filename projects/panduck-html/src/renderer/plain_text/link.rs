use super::*;
use notedown_ast::nodes::{HyperLink, ImageLink, ResourceDescriptor, TagReference, TwoWayLink};

impl PlainHTML for SmartLink {
    fn plain_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        match self {
            Self::EMail(v) => v.plain_html(f),
            Self::Normal(v) => v.plain_html(f),
            Self::Image(v) => v.plain_html(f),
            Self::TwoWay(v) => v.plain_html(f),
            Self::Reference(v) => v.plain_html(f),
            Self::ExternalResource(v) => v.plain_html(f),
        }
    }
}

impl PlainHTML for ResourceDescriptor {
    fn plain_html(&self, _: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}

impl PlainHTML for EmailLink {
    fn plain_html(&self, _: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}

impl PlainHTML for HyperLink {
    fn plain_html(&self, _: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}
impl PlainHTML for ImageLink {
    fn plain_html(&self, _: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}
impl PlainHTML for TwoWayLink {
    fn plain_html(&self, _: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}
impl PlainHTML for TagReference {
    fn plain_html(&self, _: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}
