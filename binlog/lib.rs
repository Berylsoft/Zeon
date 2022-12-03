use std::{io, marker::Unpin};
use futures_lite::{AsyncWrite, AsyncWriteExt, AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
use zeon::{std::codegen::meta::Commit, meta::{CommitPtr, CommitIndexItem, binlog::*}, util::u64_usize};

macro_rules! index_io_iter_may_end {
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
        let (index, content) = write_commit(commit);
        self.content.write_all(&content).await.map_err(Error::ContentIo)?;
        self.content.flush().await.map_err(Error::ContentIo)?;
        self.index.write_all(&index).await.map_err(Error::IndexIo)?;
        self.index.flush().await.map_err(Error::IndexIo)?;
        Ok(())
    }
}

#[inline]
async fn read_fixed<F: AsyncRead + Unpin, const N: usize>(f: &mut F) -> io::Result<[u8; N]> {
    let mut buf = [0u8; N];
    f.read_exact(&mut buf).await?;
    Ok(buf)
}

#[inline]
async fn read_vec<F: AsyncRead + Unpin>(f: &mut F, len: usize) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; len];
    f.read_exact(&mut buf).await?;
    Ok(buf)
}

pub struct Reader<F: AsyncRead + Unpin> {
    index: F,
    content: F,
}

impl<F: AsyncRead + Unpin> Reader<F> {
    pub async fn init(mut index: F, mut content: F) -> Result<Reader<F>> {
        check_index_ident(read_fixed(&mut index).await.map_err(Error::IndexIo)?)?;
        check_content_ident(read_fixed(&mut content).await.map_err(Error::ContentIo)?)?;
        Ok(Reader { index, content })
    }

    async fn read_commit_inner(&mut self, CommitIndexItem { ptr, len, hash }: CommitIndexItem) -> Result<(Commit, Hash)> {
        check_commit(ptr, hash, read_vec(&mut self.content, u64_usize(len)).await.map_err(Error::IndexIo)?)
    }

    pub async fn read_commit(&mut self) -> Option<Result<(Commit, Hash)>> {
        let index = CommitIndexItem::from_bytes(index_io_iter_may_end!(read_fixed(&mut self.index)));
        Some(self.read_commit_inner(index).await)
    }
}

pub struct IndexReader<F: AsyncRead + Unpin> {
    index: F,
}

impl<F: AsyncRead + Unpin> IndexReader<F> {
    pub async fn init(mut index: F) -> Result<IndexReader<F>> {
        check_index_ident(read_fixed(&mut index).await.map_err(Error::IndexIo)?)?;
        Ok(IndexReader { index })
    }

    pub async fn read_commit(&mut self) -> Option<Result<CommitIndexItem>> {
        Some(Ok(CommitIndexItem::from_bytes(index_io_iter_may_end!(read_fixed(&mut self.index)))))
    }
}

pub async fn read_all_index<F: AsyncRead + Unpin>(index: F) -> Result<MemoryIndex> {
    let mut index = IndexReader::init(index).await?;
    let mut items = Vec::new();
    while let Some(item) = index.read_commit().await {
        items.push(item?);
    }
    Ok(MemoryIndex::new(items))
}

pub struct IndexedReader<F: AsyncRead + AsyncSeek + Unpin> {
    index: MemoryIndex,
    content: F,
    dirty: bool,
}

impl<F: AsyncRead + AsyncSeek + Unpin> IndexedReader<F> {
    pub async fn init(mut index: MemoryIndex, mut content: F) -> Result<IndexedReader<F>> {
        index.clear_iter_state();
        content.seek(io::SeekFrom::Start(0)).await.map_err(Error::ContentIo)?;
        check_content_ident(read_fixed(&mut content).await.map_err(Error::ContentIo)?)?;
        Ok(IndexedReader { index, content, dirty: false })
    }

    async fn read_commit_inner(&mut self, dirt: bool, (offset, CommitIndexItem { ptr, len, hash }): (u64, CommitIndexItem)) -> Result<(Commit, Hash)> {
        // dirt dirty -> dirty
        // T T -seek-> T
        // T F -seek-> T
        // F T -seek-> F
        // F F ------> F
        if dirt || self.dirty {
            self.content.seek(io::SeekFrom::Start(offset)).await.map_err(Error::ContentIo)?;
        }
        if dirt {
            self.dirty = true;
        } else if self.dirty {
            self.dirty = false;
        }
        check_commit(ptr, hash, read_vec(&mut self.content, u64_usize(len)).await.map_err(Error::ContentIo)?)
    }

    #[inline]
    pub async fn find(&mut self, ptr: CommitPtr) -> Option<Result<(Commit, Hash)>> {
        let find = self.index.find(ptr)?;
        Some(self.read_commit_inner(true, find).await)
    }

    #[inline]
    pub async fn find_by_hash(&mut self, hash: Hash) -> Option<Result<(Commit, Hash)>> {
        let find = self.index.find_by_hash(hash)?;
        Some(self.read_commit_inner(true, find).await)
    }

    #[inline]
    pub async fn read_next(&mut self) -> Option<Result<(Commit, Hash)>> {
        let next = self.index.next()?;
        Some(self.read_commit_inner(false, next).await)
    }
}
