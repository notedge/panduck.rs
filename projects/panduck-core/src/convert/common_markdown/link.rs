use super::*;
use comrak::nodes::NodeLink;


impl ToNotedown for NodeLink {
    fn into_notedown(self) -> ASTNode {
        ASTNode::default()
    }
}
