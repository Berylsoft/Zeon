macros::bin_struct! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Timestamp {
        pub secs: i64,
        pub nanos: u32,
    }
}

impl Timestamp {
    pub const EPOCH_AFTER_UNIX_EPOCH_SEC: i64 = 978307200;

    pub fn now() -> Timestamp {
        let dur = crate::util::now_raw();
        assert!(dur.as_secs() <= i64::MAX as _);
        Timestamp {
            secs: dur.as_secs() as i64 - Timestamp::EPOCH_AFTER_UNIX_EPOCH_SEC,
            nanos: dur.subsec_nanos(),
        }
    }

    pub const fn from_unix_ms(ts: i64) -> Timestamp {
        Timestamp {
            secs: ts / 1_000 - Timestamp::EPOCH_AFTER_UNIX_EPOCH_SEC,
            nanos: ((ts % 1_000) as u32) * 1_000_000,
        }
    }

    pub const fn to_unix_ms(&self) -> i64 {
        (self.secs + Timestamp::EPOCH_AFTER_UNIX_EPOCH_SEC) * 1_000 + (self.nanos / 1_000_000) as i64
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

impl TypePtr {
    pub const fn from_u16(n: u16) -> TypePtr {
        assert!(crate::util::check_stdptr(n));
        TypePtr::Std(StdPtr(n))
    }

    pub const fn from_u16_unchecked(n: u16) -> TypePtr {
        TypePtr::Std(StdPtr(n))
    }

    pub fn from_path(path: &str) -> TypePtr {
        TypePtr::Hash(crate::util::shake256(path.as_bytes()))
    }

    pub const fn as_std(self) -> Option<StdPtr> {
        match self {
            Self::Std(stdptr) => Some(stdptr),
            _ => None,
        }
    }

    pub const fn as_std_inner(self) -> Option<u16> {
        match self {
            Self::Std(StdPtr(n)) => Some(n),
            _ => None,
        }
    }

    pub const fn as_hash(self) -> Option<[u8; 7]> {
        match self {
            Self::Hash(hash) => Some(hash),
            _ => None,
        }
    }
}

impl StdPtr {
    pub const fn from_u16(n: u16) -> StdPtr {
        assert!(crate::util::check_stdptr(n));
        StdPtr(n)
    }

    #[inline]
    pub const fn from_u16_unchecked(n: u16) -> StdPtr {
        StdPtr(n)
    }

    #[inline]
    pub const fn to_u16(self) -> u16 {
        self.0
    }
}
