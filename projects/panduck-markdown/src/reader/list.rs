use super::*;
use markdown::mdast::BlockQuote;
use wasi_notedown::exports::notedown::core::syntax_tree::{ListEnvironment, ListItem};

impl NoteBlock for List {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let list = ListEnvironment { items: vec![], range: self.position.as_range() };

        for x in self.children {
            match x {
                Node::ListItem(v) => {
                    let items = root_items(v.children, state)?;
                    ListItem { level: 0, checked: v.checked, range: v.position.as_range() };
                }
                _ => unreachable!(),
            };
        }
        Ok(RootItem::List(list))
    }
}

impl NoteBlock for BlockQuote {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let list = ListEnvironment { items: vec![], range: self.position.as_range() };

        Ok(RootItem::List(list))
    }
}
