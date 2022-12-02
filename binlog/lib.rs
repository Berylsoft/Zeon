#![allow(unused_macros)]

pub const BS_IDENT_INDEX:   u32 = 0x42650100;
pub const BS_IDENT_CONTENT: u32 = 0x42650200;

use std::{io, marker::Unpin};
use futures_lite::{AsyncWrite, AsyncWriteExt, AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
use zeon::{std::codegen::meta::Commit, types::{Value, DecodeError, Schema}, meta::{CommitPtr, CommitIndexItem}};

type Hash = [u8; 32];

macros::error_enum! {
    #[derive(Debug)]
    Error {
        Ident(u32, u32),
        Unorder(CommitPtr, CommitPtr),
        Hash(Hash, Hash),
        IndexIo(io::Error),
        ContentIo(io::Error),
        Duplicate(CommitPtr),
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

macro_rules! try_opt {
    ($res:expr) => {
        match $res {
            Some(res) => res,
            None => return Ok(None),
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
        iter_check!(ptr, commit.ptr, Unorder);
        Some(Ok((commit, hash)))
    }
}

pub struct IndexReader<F: AsyncRead + Unpin> {
    index: F,
}

impl<F: AsyncRead + Unpin> IndexReader<F> {
    pub async fn init(mut index: F) -> Result<IndexReader<F>> {
        let mut ident = [0u8; 4];
        index_try!(index.read_exact(&mut ident));
        check!(u32::from_be_bytes(ident), BS_IDENT_INDEX, Ident);
        Ok(IndexReader { index })
    }

    pub async fn read_commit(&mut self) -> Option<Result<CommitIndexItem>> {
        let mut index = [0u8; CommitIndexItem::SIZE];
        index_try_iter_may_end!(self.index.read_exact(&mut index));
        Some(Ok(CommitIndexItem::from_bytes(index)))
    }
}

pub struct IndexedReader<F: AsyncRead + AsyncSeek + Unpin> {
    index: Vec<CommitIndexItem>,
    content: F,
    pos: usize,
    dirty: bool,
}

impl<F: AsyncRead + AsyncSeek + Unpin> IndexedReader<F> {
    pub async fn init<F2: AsyncRead + Unpin>(index: F2, mut content: F) -> Result<IndexedReader<F>> {
        let mut index_f = IndexReader::init(index).await?;
        let mut index = Vec::new();
        while let Some(index_item) = index_f.read_commit().await {
            index.push(index_item?);
        }
        content_try!(content.seek(io::SeekFrom::Start(0)));
        let mut ident = [0u8; 4];
        content_try!(content.read_exact(&mut ident));
        check!(u32::from_be_bytes(ident), BS_IDENT_CONTENT, Ident);
        Ok(IndexedReader { index, content, pos: 0, dirty: false })
    }

    pub fn index_find(&self, ptr: CommitPtr) -> Option<(u64, CommitIndexItem)> {
        let mut offset = 0;
        for index_item in &self.index {
            if ptr != index_item.ptr {
                offset += index_item.len;
            } else {
                return Some((offset, index_item.clone()));
            }
        }
        None
    }

    pub async fn find(&mut self, ptr: CommitPtr) -> Result<Option<(Commit, Hash)>> {
        let (offset, CommitIndexItem { ptr, len, hash }) = try_opt!(self.index_find(ptr));
        self.dirty = true;
        content_try!(self.content.seek(io::SeekFrom::Start(offset)));
        let len = len.try_into().expect("FATAL: u64 length to usize error");
        let mut content = vec![0u8; len];
        content_try!(self.content.read_exact(&mut content));
        let _hash = zeon::util::shake256(&content);
        check!(hash, _hash, Hash);
        let commit = Commit::deserialize(Value::decode(&content)?);
        check!(ptr, commit.ptr, Unorder);
        Ok(Some((commit, hash)))
    }
}
