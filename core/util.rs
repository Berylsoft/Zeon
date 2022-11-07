pub fn shake256<const N: usize>(bytes: &[u8]) -> [u8; N] {
    use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};
    let mut hasher = Shake256::default();
    hasher.update(bytes);
    let mut reader = hasher.finalize_xof();
    let mut res = [0u8; N];
    reader.read(&mut res);
    res
}

pub fn now_raw() -> std::time::Duration {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

#[inline]
pub fn zigzag_encode(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

#[inline]
pub fn zigzag_decode(n: u64) -> i64 {
    ((n >> 1) ^ (-((n & 1) as i64)) as u64) as i64
}

#[inline]
pub fn from_h4l4(h4: u8, l4: u8) -> u8 {
    debug_assert!(h4 <= 0xF);
    debug_assert!(l4 <= 0xF);
    h4 << 4 | l4
}

#[inline]
pub fn to_h4l4(n: u8) -> (u8, u8) {
    (n >> 4, n & 0xF)
}
