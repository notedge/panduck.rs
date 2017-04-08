use super::*;
use comrak::nodes::NodeList;


impl ToNotedown for NodeList {
    fn into_notedown(self) -> ASTNode {
        ASTNode::default()
    }
}