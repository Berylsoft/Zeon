#![allow(unused_macros)]

use super::*;
use std::io::{Read, Cursor};

macro_rules! decode_macros {
    ($buf:expr) => {
        macro_rules! conv_tag {
            ($u8:expr) => {{
                let tag: Tag = $u8.try_into()?;
                tag
            }};
        }
        macro_rules! u8 {
            () => {{
                let [b] = sized_bytes!(1);
                b
            }};
        }
        macro_rules! bytes {
            ($size:expr) => {{
                let mut _buf = vec![0u8; $size];
                $buf.read_exact(&mut _buf)?;
                _buf
            }};
        }
        macro_rules! sized_bytes {
            ($len:expr) => {{
                let mut _buf = [0u8; $len];
                $buf.read_exact(&mut _buf)?;
                _buf
            }};
        }
        macro_rules! fixed_u16 {
            () => {
                u16::from_be_bytes(sized_bytes!(2))
            };
        }
        macro_rules! fixed_u32 {
            () => {
                u32::from_be_bytes(sized_bytes!(4))
            };
        }
        macro_rules! fixed_u64 {
            () => {
                u64::from_be_bytes(sized_bytes!(8))
            };
        }
        macro_rules! fixed_f64 {
            () => {
                f64::from_be_bytes(sized_bytes!(8))
            };
        }
        macro_rules! tag {
            () => {
                conv_tag!(u8!())
            };
        }
        macro_rules! tag_with_noop {
            ($l4:expr) => {
                assert_eq!($l4, 0)
            };
        }
        macro_rules! tag_with_bool {
            ($l4:expr) => {
                match $l4 {
                    0 => false,
                    1 => true,
                    _ => unreachable!(),
                }
            };
        }
        macro_rules! tag_with_uvar {
            ($l4:expr) => {
                match $l4 {
                    L4_EXT_U8 => u8!() as u64,
                    L4_EXT_U16 => fixed_u16!() as u64,
                    L4_EXT_U32 => fixed_u32!() as u64,
                    L4_EXT_U64 => fixed_u64!(),
                    s => s as u64,
                }
            };
        }
        macro_rules! tag_with_ivar {
            ($l4:expr) => {
                crate::util::zigzag_decode(tag_with_uvar!($l4))
            };
        }
        macro_rules! tag_with_szvar {
            ($l4:expr) => {{
                let size: usize = tag_with_uvar!($l4).try_into()?;
                size
            }};
        }
        macro_rules! typeptr {
            () => {{
                let h8 = u8!();
                match h8 {
                    0xFF => {
                        let hash = sized_bytes!(7);
                        TypePtr::Hash(hash)
                    },
                    h8 => {
                        let l8 = u8!();
                        TypePtr::Std(StdPtr(u16::from_be_bytes([h8, l8])))
                    },
                }
            }};
        }
        macro_rules! comptype {
            () => {
                Type::decode_from($buf)?
            };
        }
        macro_rules! value {
            () => {
                Value::decode_from($buf)?
            };
        }
        macro_rules! seq {
            ($size:expr) => {{
                let mut s = Vec::with_capacity($size);
                for _ in 0..$size {
                    s.push(value!())
                }
                s
            }};
        }
        macro_rules! seq_map {
            ($size:expr) => {{
                let mut s = Vec::with_capacity($size);
                for _ in 0..$size {
                    s.push((value!(), value!()))
                }
                s
            }};
        }
    };
}

impl Type {
    pub fn decode_from(buf: &mut Cursor<&[u8]>) -> Result<Type, DecodeError> {
        decode_macros!(buf);
        let tag = tag!();
        Ok(match tag {
            Tag::Unit => Type::Unit,
            Tag::Bool => Type::Bool,
            Tag::Int => Type::Int,
            Tag::UInt => Type::UInt,
            Tag::Float => Type::Float,
            Tag::String => Type::String,
            Tag::Bytes => Type::Bytes,
            Tag::Type => Type::Type,
            Tag::ObjectRef => Type::ObjectRef,

            Tag::Option => {
                let t = comptype!();
                Type::Option(Box::new(t))
            },
            Tag::List => {
                let t = comptype!();
                Type::List(Box::new(t))
            },
            Tag::Map => {
                let tk = comptype!();
                let tv = comptype!();
                Type::Map(Box::new(tk), Box::new(tv))
            },

            Tag::Alias => {
                let ptr = typeptr!();
                Type::Alias(ptr)
            },
            Tag::Enum => {
                let ptr = typeptr!();
                Type::Enum(ptr)
            },
            Tag::Tuple => {
                let ptr = typeptr!();
                Type::Tuple(ptr)
            },
            Tag::Struct => {
                let ptr = typeptr!();
                Type::Struct(ptr)
            },
        })
    }

    pub fn decode(buf: &[u8]) -> Result<Type, DecodeError> {
        let mut cur = Cursor::new(buf);
        Type::decode_from(&mut cur)
    }
}

impl Value {
    pub fn decode_from(buf: &mut Cursor<&[u8]>) -> Result<Value, DecodeError> {
        decode_macros!(buf);
        let (tag, l4) = crate::util::to_h4l4(u8!());
        let tag: Tag = conv_tag!(tag);
        Ok(match tag {
            Tag::Unit => {
                tag_with_noop!(l4);
                Value::Unit
            },
            Tag::Bool => {
                let b = tag_with_bool!(l4);
                Value::Bool(b)
            },
            Tag::Int => {
                let i = tag_with_ivar!(l4);
                Value::Int(i)
            },
            Tag::UInt => {
                let u = tag_with_uvar!(l4);
                Value::UInt(u)
            },
            Tag::Float => {
                tag_with_noop!(l4);
                let f = fixed_f64!();
                Value::Float(f)
            },
            Tag::String => {
                let len = tag_with_szvar!(l4);
                let b = bytes!(len);
                Value::String(String::from_utf8(b)?)
            },
            Tag::Bytes => {
                let len = tag_with_szvar!(l4);
                let b = bytes!(len);
                Value::Bytes(b)
            },
            Tag::Option => {
                let b = tag_with_bool!(l4);
                let t = comptype!();
                let opt = if b {
                    Some(value!())
                } else { None };
                Value::Option(t, Box::new(opt))
            },
            Tag::List => {
                let len = tag_with_szvar!(l4);
                let t = comptype!();
                let s = seq!(len);
                Value::List(t, s)
            },
            Tag::Map => {
                let len = tag_with_szvar!(l4);
                let tk = comptype!();
                let tv = comptype!();
                let s = seq_map!(len);
                Value::Map((tk, tv), s)
            },
            Tag::Alias => {
                tag_with_noop!(l4);
                let ptr = typeptr!();
                let v = value!();
                Value::Alias(ptr, Box::new(v))
            },
            Tag::Enum => {
                let ev = tag_with_uvar!(l4) as u8;
                let ptr = typeptr!();
                let v = value!();
                Value::Enum(ptr, ev, Box::new(v))
            },
            Tag::Tuple => {
                let len = tag_with_szvar!(l4);
                let ptr = typeptr!();
                let s = seq!(len);
                Value::Tuple(ptr, s)
            },
            Tag::Struct => {
                let len = tag_with_szvar!(l4);
                let ptr = typeptr!();
                let s = seq!(len);
                Value::Struct(ptr, s)
            },
            Tag::Type => {
                tag_with_noop!(l4);
                let t = comptype!();
                Value::Type(t)
            },
            Tag::ObjectRef => {
                tag_with_noop!(l4);
                let ot = fixed_u16!();
                let oid = fixed_u64!();
                Value::ObjectRef(ot, oid)
            },
        })
    }

    pub fn decode(buf: &[u8]) -> Result<Value, DecodeError> {
        let mut cur = Cursor::new(buf);
        Value::decode_from(&mut cur)
    }
}
