use super::*;
use comrak::nodes::{NodeCode, NodeCodeBlock};

impl ToNotedown for NodeCodeBlock {
    fn into_notedown(self) -> ASTNode {
        todo!()
    }
}

impl ToNotedown for NodeCode {
    fn into_notedown(self) -> ASTNode {
        todo!()
    }
}
