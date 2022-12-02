pub const BS_IDENT_INDEX:   u32 = 0x42650100;
pub const BS_IDENT_CONTENT: u32 = 0x42650200;

use crate::{meta::{CommitPtr, CommitIndexItem}, types::{Value, Schema, DecodeError}, std::codegen::meta::Commit};

pub type Hash = [u8; 32];

macros::error_enum! {
    #[derive(Debug)]
    Error {
        Ident(u32, u32),
        Unorder(CommitPtr, CommitPtr),
        Hash(Hash, Hash),
        IndexIo(std::io::Error),
        ContentIo(std::io::Error),
        Duplicate(CommitPtr),
    } convert {
        Decode => DecodeError,
    }
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! check {
    ($l:expr, $r:expr, $varient:ident) => {
        if $l != $r {
            return Err(Error::$varient($l, $r));
        }
    };
}

#[inline]
pub fn check_index_ident(ident: [u8; 4]) -> Result<()> {
    check!(u32::from_be_bytes(ident), BS_IDENT_INDEX, Ident);
    Ok(())
}

#[inline]
pub fn check_content_ident(ident: [u8; 4]) -> Result<()> {
    check!(u32::from_be_bytes(ident), BS_IDENT_CONTENT, Ident);
    Ok(())
}

pub fn write_commit(commit: Commit) -> ([u8; CommitIndexItem::SIZE], Vec<u8>) {
    let ptr = commit.ptr.clone();
    let content = commit.serialize().encode();
    let len = crate::util::usize_u64(content.len());
    let hash = crate::util::shake256(&content);
    let index = CommitIndexItem { ptr, len, hash }.to_bytes();
    (index, content)
}

pub fn check_commit(ptr: CommitPtr, hash: Hash, content: Vec<u8>) -> Result<(Commit, Hash)> {
    let _hash = crate::util::shake256(&content);
    check!(hash, _hash, Hash);
    let commit = Commit::deserialize(Value::decode(&content)?);
    check!(ptr, commit.ptr, Unorder);
    Ok((commit, hash))
}

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
