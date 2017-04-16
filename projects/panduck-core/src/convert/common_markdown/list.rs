use super::*;
use comrak::nodes::{NodeDescriptionItem, NodeList};

impl ToNotedown for NodeList {
    fn into_notedown(self) -> ASTNode {
        ASTNode::default()
    }
}

impl ToNotedown for NodeDescriptionItem {
    fn into_notedown(self) -> ASTNode {
        ASTNode::default()
    }
}
