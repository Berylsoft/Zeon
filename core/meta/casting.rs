#![allow(unused_macros)]

use super::*;

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

/*
impl TypePtr {
    pub const SIZE: usize = 8;

    pub fn from_bytes(raw: [u8; Self::SIZE]) -> Self {
        match raw[0] {
            0x00 => TypePtr::Std(StdPtr::from_u16(u16::from_be_bytes(raw[6..7].try_into().unwrap()))),
            0xFF => TypePtr::Hash(raw[1..7].try_into().unwrap()),
            _ => unreachable!(),
        }
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
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
*/

impl From<u8> for RevType {
    fn from(b: u8) -> Self {
        match b {
            0 => RevType::Const,
            1 => RevType::Mut,
            2 => RevType::IterListAdd,
            3 => RevType::IterSetAdd,
            4 => RevType::IterSetRemove,
            5 => RevType::Complex,
            _ => unreachable!(),
        }
    }
}

impl From<RevType> for u8 {
    fn from(s: RevType) -> Self {
        match s {
            RevType::Const => 0,
            RevType::Mut => 1,
            RevType::IterListAdd => 2,
            RevType::IterSetAdd => 3,
            RevType::IterSetRemove => 4,
            RevType::Complex => 5,
        }
    }
}

macro_rules! bin_struct_complex {
    {
        $struct_name:ident { $($field:ident -> $_ftyk:tt($_ftyv:tt))* }
    } => {
        macro_rules! size {
            (Number($ty:ty)) => {
                std::mem::size_of::<$ty>()
            };
            (Bytes($size:literal)) => {
                $size
            };
            (Struct($ty:ty)) => {
                <$ty>::SIZE
            };
        }

        impl $struct_name {
            pub const SIZE: usize = $(size!($_ftyk($_ftyv))+)*0;

            pub fn from_bytes(raw: [u8; Self::SIZE]) -> Self {
                let mut offset: usize = 0;
                $(
                    let $field = {
                        let _size = size!($_ftyk($_ftyv));
                        let _slice = raw[offset..(offset + _size)].try_into().unwrap();
                        macro_rules! from {
                            (Number($ty:ty)) => {
                                <$ty>::from_be_bytes(_slice)
                            };
                            (Bytes($size:literal)) => {
                                _slice
                            };
                            (Struct($ty:ty)) => {
                                <$ty>::from_bytes(_slice)
                            };
                        }
                        let _val = from!($_ftyk($_ftyv));
                        offset += _size;
                        _val
                    };
                )*
                assert_eq!(offset, Self::SIZE);
                Self { $($field,)* }
            }

            pub fn to_bytes(&self) -> [u8; Self::SIZE] {
                let mut buf = [0u8; Self::SIZE];
                let mut offset: usize = 0;
                $({
                    let _size = size!($_ftyk($_ftyv));
                    macro_rules! to {
                        (Number($ty:ty)) => {
                            self.$field.to_be_bytes()
                        };
                        (Bytes($size:literal)) => {
                            self.$field
                        };
                        (Struct($ty:ty)) => {
                            self.$field.to_bytes()
                        };
                    }
                    let _slice = to!($_ftyk($_ftyv));
                    (&mut buf[offset..(offset + _size)]).copy_from_slice(&_slice);
                    offset += _size;
                })*
                assert_eq!(offset, Self::SIZE);
                buf
            }
        }
    };
}

bin_struct_complex! {
    CommitPtr {
        ts  -> Struct(Timestamp)
        opr -> Struct(ObjectPtr)
        seq -> Number(u16)
    }
}

bin_struct_complex! {
    CommitIndexItem {
        ptr  -> Struct(CommitPtr)
        len  -> Number(u64)
        hash -> Bytes(32)
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
                println!("{}", hex::encode($exp.as_slice()));
                println!("{}", hex::encode(&buf));
                assert_eq!(&buf, $exp.as_slice());
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
            hash: crate::util::shake256(&[]),
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
