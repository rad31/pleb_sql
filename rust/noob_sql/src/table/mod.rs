use column::Column;

pub mod column;
pub mod data_type;

pub struct Table {
    pub rows: u64,
    pub row_id: u64,
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            name,
            rows: 0,
            row_id: 0,
            columns: vec![],
        }
    }
}
