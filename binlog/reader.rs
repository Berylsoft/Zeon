use crate::*;

macro_rules! check {
    ($l:expr, $r:expr, $varient:ident) => {
        if $l != $r {
            return Err(Error::$varient($l, $r));
        }
    };
}

async fn index_check_ident<F: AsyncRead + Unpin>(f: &mut F) -> Result<()> {
    let mut buf = [0; 4];
    f.read_exact(&mut buf).await.map_err(Error::IndexIo)?;
    check!(u32::from_be_bytes(buf), BS_IDENT_INDEX, Ident);
    Ok(())
}

async fn content_check_ident<F: AsyncRead + Unpin>(f: &mut F) -> Result<()> {
    let mut buf = [0; 4];
    f.read_exact(&mut buf).await.map_err(Error::ContentIo)?;
    check!(u32::from_be_bytes(buf), BS_IDENT_CONTENT, Ident);
    Ok(())
}

async fn index_read_index<F: AsyncRead + Unpin>(f: &mut F) -> Option<Result<CommitIndexItem>> {
    let mut buf = [0; CommitIndexItem::SIZE];
    match f.read_exact(&mut buf).await {
        Ok(()) => Some(Ok(CommitIndexItem::from_bytes(buf))),
        Err(err) => {
            if err.kind() == io::ErrorKind::UnexpectedEof {
                None
            } else {
                Some(Err(Error::IndexIo(err)))
            }
        },
    }
}

async fn content_read_commit<F: AsyncRead + Unpin>(f: &mut F, CommitIndexItem { ptr, len, hash }: CommitIndexItem) -> Result<(Commit, Hash)> {
    let mut content = vec![0; zeon::util::u64_usize(len)];
    f.read_exact(&mut content).await.map_err(Error::ContentIo)?;
    let _hash = zeon::util::shake256(&content);
    check!(hash, _hash, Hash);
    let commit = Commit::deserialize(Value::decode(&content)?);
    check!(ptr, commit.ptr, Unorder);
    Ok((commit, hash))
}

#[inline]
async fn content_seek<F: AsyncRead + AsyncSeek + Unpin>(f: &mut F, offset: u64) -> Result<()> {
    let _current = f.seek(io::SeekFrom::Start(offset)).await.map_err(Error::ContentIo)?;
    Ok(())
}

pub struct Reader<F: AsyncRead + Unpin> {
    index: F,
    content: F,
}

impl<F: AsyncRead + Unpin> Reader<F> {
    #[inline]
    pub async fn init(mut index: F, mut content: F) -> Result<Reader<F>> {
        index_check_ident(&mut index).await?;
        content_check_ident(&mut content).await?;
        Ok(Reader { index, content })
    }

    #[inline]
    pub async fn read_commit(&mut self) -> Option<Result<(Commit, Hash)>> {
        match index_read_index(&mut self.index).await {
            Some(Ok(index)) => Some(content_read_commit(&mut self.content, index).await),
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

pub struct IndexReader<F: AsyncRead + Unpin> {
    index: F,
}

impl<F: AsyncRead + Unpin> IndexReader<F> {
    #[inline]
    pub async fn init(mut index: F) -> Result<IndexReader<F>> {
        index_check_ident(&mut index).await?;
        Ok(IndexReader { index })
    }

    #[inline]
    pub async fn read_commit(&mut self) -> Option<Result<CommitIndexItem>> {
        index_read_index(&mut self.index).await
    }

    pub async fn read_all(mut self) -> Result<MemoryIndex> {
        let mut items = Vec::new();
        while let Some(item) = self.read_commit().await {
            items.push(item?);
        }
        Ok(MemoryIndex::from_inner(items))
    }
}

pub struct IndexedReader<F: AsyncRead + AsyncSeek + Unpin> {
    index: MemoryIndex,
    content: F,
    dirty: bool,
}

impl<F: AsyncRead + AsyncSeek + Unpin> IndexedReader<F> {
    pub async fn init(mut index: MemoryIndex, mut content: F) -> Result<IndexedReader<F>> {
        index.clear_iter_state();
        content_seek(&mut content, 0).await?;
        content_check_ident(&mut content).await?;
        Ok(IndexedReader { index, content, dirty: false })
    }

    async fn read_commit_inner(&mut self, dirt: bool, (offset, index): (u64, CommitIndexItem)) -> Result<(Commit, Hash)> {
        // dirt dirty -> dirty
        // T T -seek-> T
        // T F -seek-> T
        // F T -seek-> F
        // F F ------> F
        if dirt || self.dirty {
            content_seek(&mut self.content, offset).await?;
        }
        if dirt {
            self.dirty = true;
        } else if self.dirty {
            self.dirty = false;
        }
        content_read_commit(&mut self.content, index).await
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
