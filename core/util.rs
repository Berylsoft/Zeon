pub fn shake256_once<const N: usize>(bytes: &[u8]) -> [u8; N] {
    use tiny_keccak::{Shake, Hasher, Xof};
    let mut hasher = Shake::v256();
    hasher.update(bytes);
    let mut res = [0; N];
    hasher.squeeze(&mut res);
    res
}

#[inline]
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

#[inline]
pub const fn check_stdptr(n: u16) -> bool {
    let h8 = n >> 8;
    h8 != 0xFF
}

#[inline]
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

pub fn float_find_zero(f: u64) -> usize {
    let mut buf = f.to_be_bytes();
    buf.reverse();
    for (i, b) in buf.into_iter().enumerate() {
        if b != 0 {
            return 8 - i;
        }
    }
    0
}

#[inline]
pub fn u64_usize(n: u64) -> usize {
    n.try_into().expect("FATAL: u64 length to usize error")
}

#[inline]
pub fn usize_u64(n: usize) -> u64 {
    n.try_into().expect("FATAL: usize length to u64 error")
}

macro_rules! assert_none {
    ($expr:expr) => {
        assert!(matches!($expr, None))
    };
}

pub fn btreemap_insert_all<K: Ord, V>(vec: Vec<(K, V)>, map: &mut std::collections::BTreeMap<K, V>) {
    for (k, v) in vec {
        assert_none!(map.insert(k, v));
    }
}

/// # Safety
/// 
/// This is an unchecked function. The caller needs to ensure that the length of the input slice
/// and the length of the output array (`N`) are the same.
#[inline]
pub const unsafe fn slice_to_array_unchecked<T, const N: usize>(slice: &[T]) -> &[T; N] {
    let ptr = slice.as_ptr() as *const [T; N];
    // SAFETY: this is a unchecked function
    unsafe { &*ptr }
}

type BytesReadResult<T> = Result<T, (usize, usize)>;

pub trait BytesRead<'a> {
    fn read(&mut self, buf: &mut [u8]) -> BytesReadResult<()>;
    fn steal(&mut self, sz: usize) -> BytesReadResult<&'a [u8]>;
    fn read_byte(&mut self) -> Option<u8>;

    #[inline]
    fn steal_array<const N: usize>(&mut self) -> BytesReadResult<&'a [u8; N]> {
        // SAFETY: we just stealed a slice with the same length
        Ok(unsafe { slice_to_array_unchecked(self.steal(N)?) })
    }

    #[inline]
    fn read_to_array<const N: usize>(&mut self) -> BytesReadResult<[u8; N]> {
        Ok(*self.steal_array()?)
    }

    #[inline]
    fn read_to_vec(&mut self, sz: usize) -> BytesReadResult<Vec<u8>> {
        Ok(self.steal(sz)?.to_vec())
    }
}

// Originally copied from std impl Read::read_exact for &[u8]
impl<'a> BytesRead<'a> for &'a [u8] {
    fn read(&mut self, buf: &mut [u8]) -> BytesReadResult<()> {
        if buf.len() > self.len() {
            return Err((self.len(), buf.len()));
        }
        let (a, b) = self.split_at(buf.len());

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if buf.len() == 1 {
            buf[0] = a[0];
        } else {
            buf.copy_from_slice(a);
        }

        *self = b;
        Ok(())
    }

    fn steal(&mut self, sz: usize) -> BytesReadResult<&'a [u8]> {
        if sz > self.len() {
            return Err((self.len(), sz));
        }
        let (a, b) = self.split_at(sz);

        *self = b;
        Ok(a)
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.is_empty() /* 1 > self.len() */ {
            return None /* Err((0, 1)) */;
        }
        let (a, b) = self.split_at(1);

        let byte = a[0];

        *self = b;
        Some(byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(float_find_zero(0x00_00_00_00_00_00_00_00), 0);
        assert_eq!(float_find_zero(0x01_00_00_00_00_00_00_00), 1);
        assert_eq!(float_find_zero(0x01_00_00_01_00_00_00_00), 4);
        assert_eq!(float_find_zero(0x01_00_00_01_00_00_00_01), 8);
    }
}
