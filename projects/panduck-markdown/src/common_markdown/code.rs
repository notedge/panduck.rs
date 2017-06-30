use super::*;
use comrak::nodes::{NodeCode, NodeCodeBlock, NodeHeading};

impl ToNotedown for NodeCodeBlock {
    fn into_notedown(self) -> ASTNode {
        ASTKind::code_block(String::from_utf8_lossy(&self.literal), String::from_utf8_lossy(&self.info), None)
    }
}

impl ToNotedown for NodeCode {
    fn into_notedown(self) -> ASTNode {
        ASTKind::code_inline(String::from_utf8_lossy(&self.literal), None)
    }
}

impl ToNotedown for NodeHeading {
    fn into_notedown(self) -> ASTNode {
        ASTKind::header(vec![], self.level as u8, None)
    }
}
