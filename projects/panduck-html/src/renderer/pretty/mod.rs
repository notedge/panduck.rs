use super::*;

pub trait PrettyHTML {
    fn pretty_html(&self, f: &mut HTMLRenderer) -> fmt::Result;
}

impl PrettyHTML for ASTNode {
    fn pretty_html(&self, f: &mut HTMLRenderer) -> fmt::Result {
        todo!()
    }
}
