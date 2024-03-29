use super::*;
use foundations::{byterepr_struct_impl, sha3::*};

#[inline]
pub const fn check_raw_stdptr(n: u16) -> bool {
    let h8 = n >> 8;
    h8 != 0xFF
}

impl TypePtr {
    pub const fn from_u16(n: u16) -> TypePtr {
        assert!(check_raw_stdptr(n));
        TypePtr::Std(StdPtr(n))
    }

    pub const fn from_u16_unchecked(n: u16) -> TypePtr {
        TypePtr::Std(StdPtr(n))
    }

    pub fn from_path(path: &str) -> TypePtr {
        TypePtr::Hash(shake256_once(path.as_bytes()))
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
        assert!(check_raw_stdptr(n));
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

impl ByteRepr for TypePtr {
    const SIZE: usize = 8;
    type Bytes = [u8; Self::SIZE];

    fn from_bytes(raw: [u8; Self::SIZE]) -> Self {
        if raw[0] == 0xFF {
            TypePtr::Hash(raw[1..7].try_into().unwrap())
        } else {
            TypePtr::Std(StdPtr::from_u16(u16::from_be_bytes(raw[6..7].try_into().unwrap())))
        }
    }

    fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0; Self::SIZE];
        match self {
            TypePtr::Std(stdptr) => {
                buf[6..8].copy_from_slice(&stdptr.to_u16().to_be_bytes());
            },
            TypePtr::Hash(hash) => {
                buf[0] = 0xFF;
                buf[1..8].copy_from_slice(hash);
            }
        }
        buf
    }
}

byterepr_struct_impl! {
    CommitPtr {
        ts: Timestamp,
        opr: ObjectPtr,
        seq: u16,
    }
}

byterepr_struct_impl! {
    CommitIndexItem {
        ptr: CommitPtr,
        len: u64,
        hash: [u8; 32],
    }
}

#[cfg(test)]
mod test {
    use hex_literal::hex;
    use super::*;

    #[test]
    fn test() {
        assert_eq!(CommitPtr::SIZE, 24);
        assert_eq!(CommitIndexItem::SIZE, 64);

        macro_rules! case {
            ($ty:ty, $v:expr, $exp:expr) => {{
                println!("{:?}", &$v);
                let buf = $v.clone().to_bytes();
                println!("len={}", $exp.len());
                println!("{}", hex::encode(&$exp));
                println!("len={}", buf.len());
                println!("{}", hex::encode(&buf));
                assert_eq!(&buf, &$exp);
                let v2 = <$ty>::from_bytes(buf);
                assert_eq!($v, v2);
            }};
        }

        let ptr = CommitPtr {
            ts: Timestamp { secs: 0x2937b5bf, nanos: 0x05b242d8 },
            opr: ObjectPtr { ot: 0x1234, oid: 0xabcdef00 },
            seq: 0x5678,
        };

        let index = CommitIndexItem {
            ptr: ptr.clone(),
            len: 0,
            hash: shake256_once(&[]),
        };
    
        case!(
            CommitPtr,
            ptr,
            hex!("
            000000002937b5bf05b242d8123400000000abcdef005678
            ")
        );

        case!(
            CommitIndexItem,
            index,
            hex!("
            000000002937b5bf05b242d8123400000000abcdef005678
            0000000000000000
            46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762f
            ")
        );
    }
}
