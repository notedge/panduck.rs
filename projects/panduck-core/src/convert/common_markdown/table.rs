use super::*;
use comrak::nodes::TableAlignment;

impl ToNotedown for Vec<TableAlignment> {
    fn into_notedown(self) -> ASTNode {
        ASTNode::default()
    }
}
