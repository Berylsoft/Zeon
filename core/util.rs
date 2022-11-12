pub fn shake256<const N: usize>(bytes: &[u8]) -> [u8; N] {
    use tiny_keccak::{Shake, Hasher};
    let mut hasher = Shake::v256();
    hasher.update(bytes);
    let mut res = [0u8; N];
    hasher.finalize(&mut res);
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
    assert!(h4 <= 0xF);
    assert!(l4 <= 0xF);
    h4 << 4 | l4
}

#[inline]
pub const fn to_h4l4(n: u8) -> (u8, u8) {
    (n >> 4, n & 0xF)
}

pub const fn check_stdptr(n: u16) -> bool {
    let h8 = (n >> 8) as u8;
    h8 != 0xFF
}

pub fn to_snake_case(path: &str) -> String {
    path.replace('-', "_")
}

// Copied from serde: serde_derive/src/internals/case.rs
pub fn to_pascal_case(name: &str) -> String {
    let mut pascal = String::new();
    let mut capitalize = true;
    for ch in name.chars() {
        if ch == '-' {
            capitalize = true;
        } else if capitalize {
            pascal.push(ch.to_ascii_uppercase());
            capitalize = false;
        } else {
            pascal.push(ch);
        }
    }
    pascal
}

pub const fn const_bytes_equal(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut i = 0;
    while i < lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }
        i += 1;
    }
    true
}

pub const fn const_str_equal(lhs: &str, rhs: &str) -> bool {
    const_bytes_equal(lhs.as_bytes(), rhs.as_bytes())
}
