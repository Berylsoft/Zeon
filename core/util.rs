pub fn shake256_once<const N: usize>(bytes: &[u8]) -> [u8; N] {
    use tiny_keccak::{Shake, Hasher, Xof};
    let mut hasher = Shake::v256();
    hasher.update(bytes);
    let mut res = [0; N];
    hasher.squeeze(&mut res);
    res
}

#[inline]
pub const fn check_stdptr(n: u16) -> bool {
    let h8 = n >> 8;
    h8 != 0xFF
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
