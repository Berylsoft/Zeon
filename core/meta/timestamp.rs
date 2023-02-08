use super::Timestamp;
use foundations::now::*;

#[inline]
const fn u64_i64(u: u64, s: bool) -> i64 {
    let i = u as i64;
    if s { i } else { -i }
}

const NANOS_PER_SEC: u32 = 1_000_000_000;
const NANOS_PER_MILLI: u32 = 1_000_000;
const MILLIS_PER_SEC: i64 = 1_000;

const DIFF: i64 = 978307200;

impl Timestamp {
    pub const EPOCH_AFTER_UNIX_EPOCH_SEC: i64 = DIFF;

    pub fn now() -> Timestamp {
        let (direction, secs, nanos) = now_raw();
        Timestamp {
            secs: u64_i64(secs, direction) - DIFF,
            nanos,
        }
    }

    pub const fn from_unix_ms(ts: i64) -> Timestamp {
        Timestamp {
            secs: ts / MILLIS_PER_SEC - DIFF,
            nanos: ((ts % MILLIS_PER_SEC) as u32) * NANOS_PER_MILLI,
        }
    }

    pub const fn to_unix_ms(&self) -> i64 {
        (self.secs + DIFF) * MILLIS_PER_SEC
        + (self.nanos / NANOS_PER_MILLI) as i64
    }

    pub fn from_nanos(nanos: i128) -> Timestamp {
        Timestamp {
            secs: (nanos / (NANOS_PER_SEC as i128)) as i64,
            nanos: (nanos % (NANOS_PER_SEC as i128)) as u32,
        }
    }

    pub fn to_nanos(&self) -> i128 {
        (self.secs as i128) * (NANOS_PER_SEC as i128)
        + (self.nanos as i128)
    }
}

// TODO: checked calculate?
