use crate::ToNotedown;
use document_tree::{element_categories::StructuralSubElement, Document, HasChildren, Inline};
use notedown_ast::{ASTKind, ASTNode, ASTNodes};

impl ToNotedown for Document {
    fn into_notedown(self) -> ASTNode {
        ASTKind::statements(self.into_notedown_list(), None)
    }

    fn into_notedown_list(self) -> ASTNodes {
        self.children().iter().cloned().map(|e| e.into_notedown()).collect()
    }
}

impl ToNotedown for StructuralSubElement {
    fn into_notedown(self) -> ASTNode {
        todo!()
    }
}

impl ToNotedown for Inline {
    fn into_notedown(self) -> ASTNode {}
}
