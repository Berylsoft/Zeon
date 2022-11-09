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
pub const fn zigzag_encode(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

#[inline]
pub const fn zigzag_decode(n: u64) -> i64 {
    ((n >> 1) ^ (-((n & 1) as i64)) as u64) as i64
}

#[inline]
pub const fn from_h4l4(h4: u8, l4: u8) -> u8 {
    debug_assert!(h4 <= 0xF);
    debug_assert!(l4 <= 0xF);
    h4 << 4 | l4
}

#[inline]
pub const fn to_h4l4(n: u8) -> (u8, u8) {
    (n >> 4, n & 0xF)
}

pub const fn check_stdptr(n: u16) -> bool {
    let h8 = n >> 8 as u8;
    h8 != 0xFF
}

pub fn to_rust_path(path: &str) -> String {
    use convert_case::{Case, Casing};
    path.to_case(Case::Snake)
}

pub fn to_rust_name(name: &str) -> String {
    use convert_case::{Case, Casing};
    name.to_case(Case::Pascal)
}
