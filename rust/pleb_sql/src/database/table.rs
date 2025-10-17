use std::collections::LinkedList;

use crate::database::column::Column;

pub struct Table {
    pub rows: u64,
    pub row_id: u64,
    pub name: String,
    pub columns: Vec<Column>,
    // Naming scheme: {database_id}/{table_id}/{block_id}
    next_block_id: u32,
    unused_blocks: LinkedList<u32>,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            name,
            rows: 0,
            row_id: 0,
            columns: vec![],
            next_block_id: 0,
            unused_blocks: LinkedList::new(),
        }
    }

    pub fn next_block_id(&mut self) -> u32 {
        let unused = self.unused_blocks.pop_front();
        if let Some(block_id) = unused {
            return block_id;
        }
        let block_id = self.next_block_id;
        self.next_block_id += 1;
        block_id
    }

    pub fn delete_block(&mut self, block_id: u32) {
        self.unused_blocks.push_back(block_id);
    }
}
