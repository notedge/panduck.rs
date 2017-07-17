use super::*;

impl NoteBlock for Table {
    fn note_down_block(self, state: &mut ReadState) -> Result<RootItem, NotedownError> {
        let mut rows = Vec::with_capacity(self.children.len());
        for x in self.children {
            match x {
                Node::TableRow(x) => rows.extend(table_row(x, state)),
                _ => unreachable!(),
            }
        }
        let table = TableEnvironment { rows, range: self.position.as_range() };
        Ok(RootItem::Table(table))
    }
}

fn table_row(row: markdown::mdast::TableRow, state: &mut ReadState) -> Option<TableRow> {
    let mut cells = Vec::with_capacity(row.children.len());
    for x in row.children {
        match x {
            Node::TableCell(x) => cells.extend(table_cell(x, state)),
            _ => unreachable!(),
        }
    }
    Some(TableRow { cells, range: row.position.as_range() })
}

fn table_cell(cell: markdown::mdast::TableCell, state: &mut ReadState) -> Option<TableCell> {
    let items = cell.children.note_down_inline(state);
    Some(TableCell { range: cell.position.as_range() })
}
