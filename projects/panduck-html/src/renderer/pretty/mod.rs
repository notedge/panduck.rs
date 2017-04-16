use super::*;
use pretty::RcDoc;

pub trait PrettyHTML {
    fn pretty_html(&self, f: &mut HTMLRenderer) -> RcDoc<()>;
}

impl PrettyHTML for ASTNode {
    fn pretty_html(&self, _: &mut HTMLRenderer) -> RcDoc<()> {
        todo!()
    }
}
