#![allow(unused_macros)]

pub const BS_IDENT_INDEX:   u32 = 0x42650100;
pub const BS_IDENT_CONTENT: u32 = 0x42650200;

use std::{io, marker::Unpin};
use futures_lite::{AsyncWrite, AsyncWriteExt, AsyncRead, AsyncReadExt};
use zeon::{std::codegen::meta::Commit, types::{Value, DecodeError, Schema}, meta::{CommitPtr, CommitIndexItem}};

type Hash = [u8; 32];

macros::error_enum! {
    #[derive(Debug)]
    Error {
        Ident(u32, u32),
        Ptr(CommitPtr, CommitPtr),
        Hash(Hash, Hash),
        IndexIo(io::Error),
        ContentIo(io::Error),
    } convert {
        Decode => DecodeError,
    }
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! index_try {
    ($res:expr) => {
        match $res.await {
            Ok(val) => val,
            Err(err) => return Err(Error::IndexIo(err)),
        }
    };
}

macro_rules! content_try {
    ($res:expr) => {
        match $res.await {
            Ok(val) => val,
            Err(err) => return Err(Error::ContentIo(err)),
        }
    };
}

macro_rules! index_try_iter {
    ($res:expr) => {
        match $res.await {
            Ok(val) => val,
            Err(err) => return Some(Err(Error::IndexIo(err))),
        }
    };
}

macro_rules! content_try_iter {
    ($res:expr) => {
        match $res.await {
            Ok(val) => val,
            Err(err) => return Some(Err(Error::ContentIo(err))),
        }
    };
}

macro_rules! index_try_iter_may_end {
    ($res:expr) => {
        match $res.await {
            Ok(val) => val,
            Err(err) => return {
                if err.kind() == io::ErrorKind::UnexpectedEof {
                    None
                } else {
                    Some(Err(Error::IndexIo(err)))
                }
            },
        }
    };
}

macro_rules! iter_try {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(err) => return Some(Err(err.into())),
        }
    };
}

macro_rules! check {
    ($l:expr, $r:expr, $varient:ident) => {
        if $l != $r {
            return Err(Error::$varient($l, $r));
        }
    };
}

macro_rules! iter_check {
    ($l:expr, $r:expr, $varient:ident) => {
        if $l != $r {
            return Some(Err(Error::$varient($l, $r)));
        }
    };
}

pub struct Writer<F: AsyncWrite + Unpin> {
    index: F,
    content: F,
}

impl<F: AsyncWrite + Unpin> Writer<F> {
    pub async fn init(mut index: F, mut content: F) -> Result<Writer<F>> {
        index_try!(index.write_all(&crate::BS_IDENT_INDEX.to_be_bytes()));
        content_try!(content.write_all(&crate::BS_IDENT_CONTENT.to_be_bytes()));
        Ok(Writer { index, content })
    }

    pub async fn write_commit(&mut self, commit: Commit) -> Result<()> {
        let ptr = commit.ptr.clone();
        let content = commit.serialize().encode();
        let len = content.len().try_into().expect("FATAL: usize length to u64 error");
        let hash = zeon::util::shake256(&content);
        let index = CommitIndexItem { ptr, len, hash }.to_bytes();
        content_try!(self.content.write_all(&content));
        content_try!(self.content.flush());
        index_try!(self.index.write_all(&index));
        index_try!(self.index.flush());
        Ok(())
    }
}

pub struct Reader<F: AsyncRead + Unpin> {
    index: F,
    content: F,
}

impl<F: AsyncRead + Unpin> Reader<F> {
    pub async fn init(mut index: F, mut content: F) -> Result<Reader<F>> {
        let mut ident = [0u8; 4];
        index_try!(index.read_exact(&mut ident));
        check!(u32::from_be_bytes(ident), BS_IDENT_INDEX, Ident);
        let mut ident = [0u8; 4];
        content_try!(content.read_exact(&mut ident));
        check!(u32::from_be_bytes(ident), BS_IDENT_CONTENT, Ident);
        Ok(Reader { index, content })
    }

    pub async fn read_commit(&mut self) -> Option<Result<(Commit, Hash)>> {
        let mut index = [0u8; CommitIndexItem::SIZE];
        index_try_iter_may_end!(self.index.read_exact(&mut index));
        let CommitIndexItem { ptr, len, hash } = CommitIndexItem::from_bytes(index);
        let len = len.try_into().expect("FATAL: u64 length to usize error");
        let mut content = vec![0u8; len];
        content_try_iter!(self.content.read_exact(&mut content));
        let _hash = zeon::util::shake256(&content);
        iter_check!(hash, _hash, Hash);
        let commit = Commit::deserialize(iter_try!(Value::decode(&content)));
        iter_check!(ptr, commit.ptr, Ptr);
        Some(Ok((commit, hash)))
    }
}
