use std::{io, marker::Unpin};
use futures_lite::{AsyncWrite, AsyncWriteExt, AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
use zeon::{std::codegen::meta::Commit, meta::{CommitPtr, CommitIndexItem, binlog::*}, util::u64_usize};

macro_rules! index_io {
    ($res:expr) => {
        match $res.await {
            Ok(val) => val,
            Err(err) => return Err(Error::IndexIo(err)),
        }
    };
}

macro_rules! content_io {
    ($res:expr) => {
        match $res.await {
            Ok(val) => val,
            Err(err) => return Err(Error::ContentIo(err)),
        }
    };
}

macro_rules! content_io_iter {
    ($res:expr) => {
        match $res.await {
            Ok(val) => val,
            Err(err) => return Some(Err(Error::ContentIo(err))),
        }
    };
}

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
        index_io!(index.write_all(&crate::BS_IDENT_INDEX.to_be_bytes()));
        content_io!(content.write_all(&crate::BS_IDENT_CONTENT.to_be_bytes()));
        Ok(Writer { index, content })
    }

    pub async fn write_commit(&mut self, commit: Commit) -> Result<()> {
        let (index, content) = write_commit(commit);
        content_io!(self.content.write_all(&content));
        content_io!(self.content.flush());
        index_io!(self.index.write_all(&index));
        index_io!(self.index.flush());
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
        check_index_ident(index_io!(read_fixed(&mut index)))?;
        check_content_ident(content_io!(read_fixed(&mut content)))?;
        Ok(Reader { index, content })
    }

    pub async fn read_commit(&mut self) -> Option<Result<(Commit, Hash)>> {
        let index = index_io_iter_may_end!(read_fixed(&mut self.index));
        let CommitIndexItem { ptr, len, hash } = CommitIndexItem::from_bytes(index);
        Some(check_commit(ptr, hash, content_io_iter!(read_vec(&mut self.content, u64_usize(len)))))
    }
}

pub struct IndexReader<F: AsyncRead + Unpin> {
    index: F,
}

impl<F: AsyncRead + Unpin> IndexReader<F> {
    pub async fn init(mut index: F) -> Result<IndexReader<F>> {
        check_index_ident(index_io!(read_fixed(&mut index)))?;
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
    pub async fn init<F2: AsyncRead + Unpin>(index: F2, mut content: F) -> Result<IndexedReader<F>> {
        let index = read_all_index(index).await?;
        content_io!(content.seek(io::SeekFrom::Start(0)));
        check_content_ident(content_io!(read_fixed(&mut content)))?;
        Ok(IndexedReader { index, content, dirty: false })
    }

    #[inline]
    async fn dirt(&mut self, offset: u64) -> io::Result<()> {
        self.content.seek(io::SeekFrom::Start(offset)).await?;
        self.dirty = true;
        Ok(())
    }

    #[inline]
    async fn clean(&mut self, offset: u64) -> io::Result<()> {
        if self.dirty {
            self.content.seek(io::SeekFrom::Start(offset)).await?;
            self.dirty = false;
        }
        Ok(())
    }

    pub async fn find(&mut self, ptr: CommitPtr) -> Option<Result<(Commit, Hash)>> {
        let (offset, CommitIndexItem { ptr, len, hash }) = self.index.find(ptr)?;
        content_io_iter!(self.dirt(offset));
        Some(check_commit(ptr, hash, content_io_iter!(read_vec(&mut self.content, u64_usize(len)))))
    }

    pub async fn find_by_hash(&mut self, hash: Hash) -> Option<Result<(Commit, Hash)>> {
        let (offset, CommitIndexItem { ptr, len, hash }) = self.index.find_by_hash(hash)?;
        content_io_iter!(self.dirt(offset));
        Some(check_commit(ptr, hash, content_io_iter!(read_vec(&mut self.content, u64_usize(len)))))
    }

    pub async fn read_commit(&mut self) -> Option<Result<(Commit, Hash)>> {
        let (offset, CommitIndexItem { ptr, len, hash }) = self.index.next()?;
        content_io_iter!(self.clean(offset));
        Some(check_commit(ptr, hash, content_io_iter!(read_vec(&mut self.content, u64_usize(len)))))
    }
}
