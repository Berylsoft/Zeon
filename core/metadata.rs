macros::bin_struct! {
    #[derive(Clone, Copy, Debug)]
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
    #[derive(Clone, Copy, Debug)]
    pub struct ObjectRef {
        pub ot: u16,
        pub oid: u64,
    }
}
