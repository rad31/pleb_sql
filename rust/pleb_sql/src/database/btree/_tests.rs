use crate::database::page_size::PageSize;

use super::BTree;

#[test]
fn btree_height_0() {
    let mut btree = BTree::new(PageSize::Debug(32), 8);
    let record = (1 as u64).to_le_bytes();
    btree.insert(&record);
    println!("{}", btree);

    let record = (2 as u64).to_le_bytes();
    btree.insert(&record);
    println!("{}", btree);

    let record = (4 as u64).to_le_bytes();
    btree.insert(&record);
    println!("{}", btree);

    let record = (3 as u64).to_le_bytes();
    btree.insert(&record);
    println!("{}", btree);
}
