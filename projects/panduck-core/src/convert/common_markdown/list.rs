use super::*;
use comrak::nodes::{ListType, NodeDescriptionItem, NodeList};
use notedown_ast::nodes::{ListSimpleNode, ListView};

impl ToNotedown for NodeList {
    fn into_notedown(self) -> ASTNode {
        match self.list_type {
            ListType::Bullet => {
                let node = ListSimpleNode::orderless_list(vec![]);
                ListView::Orderless(box node).into()
            }
            ListType::Ordered => {
                let node = ListSimpleNode::ordered_list(vec![]);
                ListView::Ordered(box node).into()
            }
        }
    }
}

impl ToNotedown for NodeDescriptionItem {
    fn into_notedown(self) -> ASTNode {
        todo!()
    }
}
