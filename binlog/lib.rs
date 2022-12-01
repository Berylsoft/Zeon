pub const BS_IDENT_INDEX:   u32 = 0x42650100;
pub const BS_IDENT_CONTENT: u32 = 0x42650200;

use std::marker::Unpin;
use futures_lite::{AsyncWrite, AsyncWriteExt, AsyncRead, AsyncReadExt, AsyncSeek};
use zeon::{std::codegen::meta::Commit, types::{Value, DecodeError, Schema}, meta::{CommitPtr, CommitIndexItem}};

type Hash = [u8; 32];

macros::error_enum! {
    #[derive(Debug)]
    Error {
        Ident(u32, u32),
        Ptr(CommitPtr, CommitPtr),
        Hash(Hash, Hash),
    } convert {
        Io => std::io::Error,
        Size => std::num::TryFromIntError,
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
        index.write_all(&crate::BS_IDENT_INDEX.to_be_bytes()).await?;
        content.write_all(&crate::BS_IDENT_CONTENT.to_be_bytes()).await?;
        Ok(Writer { index, content })
    }

    pub async fn write_commit(&mut self, commit: Commit) -> Result<()> {
        let ptr = commit.ptr.clone();
        let content = commit.serialize().encode();
        let len = content.len().try_into()?;
        let hash = zeon::util::shake256(&content);
        let index = CommitIndexItem { ptr, len, hash }.to_bytes();
        self.content.write_all(&content).await?;
        self.content.flush().await?;
        self.index.write_all(&index).await?;
        self.index.flush().await?;
        Ok(())
    }
}

pub struct Reader<F: AsyncRead + AsyncSeek + Unpin> {
    index: F,
    content: F,
}

impl<F: AsyncRead + AsyncSeek + Unpin> Reader<F> {
    pub async fn init(mut index: F, mut content: F) -> Result<Reader<F>> {
        let mut ident = [0u8; 4];
        index.read_exact(&mut ident).await?;
        if ident != BS_IDENT_INDEX.to_be_bytes() {
            return Err(Error::Ident(BS_IDENT_INDEX, u32::from_be_bytes(ident)));
        }
        content.read_exact(&mut ident).await?;
        if ident != BS_IDENT_CONTENT.to_be_bytes() {
            return Err(Error::Ident(BS_IDENT_CONTENT, u32::from_be_bytes(ident)));
        }
        Ok(Reader { index, content })
    }

    async fn read_commit_inner(&mut self) -> Result<(Commit, Hash)> {
        let mut index = [0u8; CommitIndexItem::SIZE];
        self.index.read_exact(&mut index).await?;
        let CommitIndexItem { ptr, len, hash } = CommitIndexItem::from_bytes(index);
        let len = len.try_into()?;
        let mut content = vec![0u8; len];
        self.content.read_exact(&mut content).await?;
        let _hash =  zeon::util::shake256(&content);
        if hash != _hash {
            return Err(Error::Hash(hash, _hash));
        }
        let commit = Commit::deserialize(Value::decode(&mut content)?);
        if ptr != commit.ptr {
            return Err(Error::Ptr(ptr, commit.ptr.clone()));
        }
        Ok((commit, hash))
    }

    pub async fn read_commit(&mut self) -> Option<Result<(Commit, Hash)>> {
        match self.read_commit_inner().await {
            Ok(msg) => Some(Ok(msg)),
            Err(Error::Io(err)) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    None
                } else {
                    Some(Err(Error::Io(err)))
                }
            },
            Err(err) => Some(Err(err)),
        }
    }
}
