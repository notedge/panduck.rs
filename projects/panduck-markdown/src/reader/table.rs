use markdown::mdast::{TableCell, TableRow};
use super::*;
use wasi_notedown::exports::notedown::core::syntax_tree::TableEnvironment;

impl NoteBlock for Table {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let items = root_items(self.children, state)?;

        
        let table = TableEnvironment { rows: vec![], range: self.position.as_range() };
        Ok(RootItem::Table(table))
    }
}

impl NoteBlock for TableRow {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let items = root_items(self.children, state)?;
        let table = TableEnvironment { rows: vec![], range: self.position.as_range() };
        Ok(RootItem::Table(table))
    }
}

impl NoteBlock for TableCell {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let items = root_items(self.children, state)?;
        let table = TableEnvironment { rows: vec![], range: self.position.as_range() };
        Ok(RootItem::Table(table))
    }
}
