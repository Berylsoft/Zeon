#![allow(unused_imports)]

pub const BS_IDENT_INDEX:   u32 = 0x42650100;
pub const BS_IDENT_CONTENT: u32 = 0x42650200;

use std::{io, marker::Unpin};
use futures_lite::{AsyncWrite, AsyncWriteExt, AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
use zeon::{meta::{Commit, CommitPtr, CommitIndexItem}, types::{Value, Schema, DecodeError}};

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
        // Seek(u64, u64),
    } convert {
        Decode => DecodeError,
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Writer<F: AsyncWrite + Unpin> {
    index: F,
    content: F,
}

impl<F: AsyncWrite + Unpin> Writer<F> {
    pub async fn init(mut index: F, mut content: F) -> Result<Writer<F>> {
        index.write_all(&BS_IDENT_INDEX.to_be_bytes()).await.map_err(Error::IndexIo)?;
        content.write_all(&BS_IDENT_CONTENT.to_be_bytes()).await.map_err(Error::ContentIo)?;
        Ok(Writer { index, content })
    }

    pub async fn write_commit(&mut self, commit: Commit) -> Result<()> {
        let ptr = commit.ptr.clone();
        let content = commit.serialize().encode();
        let len = zeon::util::usize_u64(content.len());
        let hash = zeon::util::shake256(&content);
        let index = CommitIndexItem { ptr, len, hash }.to_bytes();
        self.content.write_all(&content).await.map_err(Error::ContentIo)?;
        self.content.flush().await.map_err(Error::ContentIo)?;
        self.index.write_all(&index).await.map_err(Error::IndexIo)?;
        self.index.flush().await.map_err(Error::IndexIo)?;
        Ok(())
    }
}

#[cfg(feature = "reader")]
mod memory;
#[cfg(feature = "reader")]
pub use memory::MemoryIndex;
#[cfg(feature = "reader")]
mod reader;
#[cfg(feature = "reader")]
pub use reader::*;
