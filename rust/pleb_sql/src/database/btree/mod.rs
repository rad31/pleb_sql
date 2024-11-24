use std::mem::transmute;

use super::page_size::PageSize;

#[cfg(test)]
mod _tests;

pub struct BTree {
    num_items: u64,
    item_bytes: u32,
    items_per_page: u32,
    height: u32,
    page_size: u32,
    data: Vec<u8>,
}

impl BTree {
    pub fn new(page_size: PageSize, item_bytes: u32) -> BTree {
        let page_size = match page_size {
            PageSize::Eight => 8192,
            PageSize::Four => 4096,
            PageSize::Two => 2048,
            PageSize::Debug(n) => n,
        };
        BTree {
            num_items: 0,
            item_bytes,
            items_per_page: page_size / item_bytes,
            page_size,
            height: 0,
            data: vec![0; page_size as usize],
        }
    }

    pub fn insert(&mut self, record: &[u8]) {
        self.num_items += 1;
        let record_key = record[0..8].get_key();

        debug_assert_eq!(record.len(), self.item_bytes as usize);

        // root page is full
        if self.height == 0 && (self.num_items * self.item_bytes as u64) > self.page_size.into() {
            let size = self.page_size as usize;
            let mut new_data = vec![0; size * 3];
            let mid_start: usize = (self.items_per_page * self.item_bytes / 2) as usize;
            let mid_end: usize = mid_start + (self.item_bytes as usize);

            new_data[..size].copy_from_slice(&self.data[mid_start..mid_end]);
            new_data[size..(size * 2)].copy_from_slice(&self.data[..mid_start]);
            new_data[(size * 2)..(size * 3)].copy_from_slice(&self.data[mid_end..]);

            self.data = new_data;
            return;
        }

        // root page not full
        if self.height == 0 {
            let item_bytes = self.item_bytes as usize;
            let mut i = 0;
            loop {
                let key = self.data[i..i + 8].get_key();
                // item is largest in page, put at end
                if key == 0 {
                    self.data[i..i + item_bytes].copy_from_slice(record);
                    break;
                }
                // put item at position and shift elements down
                if record_key <= key {
                    let mut j = i;

                    while self.data[j..j + 8].get_key() != 0 {
                        j += item_bytes;
                    }

                    let temp = self.data[i..j].to_vec();
                    self.data[i..i + item_bytes].copy_from_slice(record);
                    self.data[i + item_bytes..j + item_bytes].copy_from_slice(&temp);

                    break;
                }
                i += item_bytes;
            }
        }
    }
}

impl std::fmt::Display for BTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        let mut first = true;
        write!(f, "[")?;
        while i < self.data.len() {
            if !first {
                write!(f, ", ")?;
            }
            let slice = &self.data[i..i + self.item_bytes as usize];
            write!(f, "{}", slice.get_key())?;
            i += self.item_bytes as usize;
            first = false;
        }
        write!(f, "]")?;
        Ok(())
    }
}

trait HasKey {
    fn get_key(self) -> u64;
}

impl HasKey for &[u8] {
    fn get_key(self) -> u64 {
        u64::from_le_bytes(self.try_into().unwrap())
    }
}
