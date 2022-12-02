macros::bin_struct! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Timestamp {
        pub secs: i64,
        pub nanos: u32,
    }
}

macros::bin_struct! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ObjectPtr {
        pub ot: u16,
        pub oid: u64,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypePtr {
    Std(StdPtr),
    Hash([u8; 7]),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StdPtr(u16);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitIndexItem {
    pub ptr: CommitPtr,
    pub len: u64,
    pub hash: [u8; 32],
}

pub use crate::std::codegen::meta::{RevType, Rev, CommitPtr};

mod casting;
pub mod binlog;
