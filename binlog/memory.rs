use crate::*;

pub struct MemoryIndex {
    items: Vec<CommitIndexItem>,
    index: usize,
    offset: u64,
}

impl MemoryIndex {
    #[inline]
    pub fn new(items: Vec<CommitIndexItem>) -> MemoryIndex {
        MemoryIndex { items, index: 0, offset: 0 }
    }

    pub fn find(&self, ptr: CommitPtr) -> Option<(u64, CommitIndexItem)> {
        let mut offset = 0;
        for index_item in &self.items {
            if ptr != index_item.ptr {
                offset += index_item.len;
            } else {
                return Some((offset, index_item.clone()));
            }
        }
        None
    }

    pub fn find_by_hash(&self, hash: Hash) -> Option<(u64, CommitIndexItem)> {
        let mut offset = 0;
        for index_item in &self.items {
            if hash != index_item.hash {
                offset += index_item.len;
            } else {
                return Some((offset, index_item.clone()));
            }
        }
        None
    }

    pub fn clear_iter_state(&mut self) {
        self.index = 0;
        self.offset = 0;
    }
}

impl Iterator for MemoryIndex {
    type Item = (u64, CommitIndexItem);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.get(self.index)?;
        let offset = self.offset;
        self.index += 1;
        self.offset += item.len;
        Some((offset, item.clone()))
    }
}
