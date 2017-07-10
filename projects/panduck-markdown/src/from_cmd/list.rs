use super::*;

impl NoteBlock for BlockQuote {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let list = ListEnvironment { items: vec![], range: self.position.as_range() };

        Ok(RootItem::List(list))
    }
}

impl NoteBlock for List {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let mut items = Vec::with_capacity(self.children.len());
        for x in self.children {
            match x {
                Node::ListItem(v) => items.extend(list_item(v, state)),
                _ => unreachable!(),
            };
        }
        let list = ListEnvironment { items, range: self.position.as_range() };
        Ok(RootItem::List(list))
    }
}

fn list_item(item: markdown::mdast::ListItem, state: &mut ReadState) -> Option<ListItem> {
    let items = root_items(item.children, state).unwrap();
    Some(ListItem { level: 0, checked: item.checked, range: item.position.as_range() })
}
