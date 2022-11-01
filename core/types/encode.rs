#![allow(unused_macros)]

use super::*;

macro_rules! encode_macros {
    ($self:expr, $buf:expr) => {
        let self_tag = $self.as_tag();
        macro_rules! conv_tag {
            ($tag:expr) => {
                $tag as u8
            };
        }
        macro_rules! u8 {
            ($c:expr) => {
                $buf.push($c)
            };
        }
        macro_rules! bytes {
            ($s:expr) => {
                $buf.extend_from_slice($s)
            };
        }
        macro_rules! fixed {
            ($n:expr) => {
                $buf.extend_from_slice($n.to_be_bytes().as_slice())
            };
        }
        macro_rules! tag {
            () => {
                u8!(conv_tag!(self_tag))
            };
        }
        macro_rules! tag_with_u4 {
            ($u4:expr) => {
                u8!(crate::util::from_h4l4(conv_tag!(self_tag), $u4))
            };
        }
        macro_rules! tag_with_noop {
            () => {
                tag_with_u4!(0)
            };
        }
        macro_rules! tag_with_bool {
            ($bool:expr) => {
                tag_with_u4!($bool as u8)
            };
        }
        macro_rules! tag_with_uvar {
            ($uvar:expr) => {
                if $uvar < L4_EXT_U8 as u64 {
                    tag_with_u4!($uvar as u8);
                } else if $uvar <= u8::MAX as u64 {
                    tag_with_u4!(L4_EXT_U8);
                    fixed!($uvar as u8);
                } else if $uvar <= u16::MAX as u64 {
                    tag_with_u4!(L4_EXT_U16);
                    fixed!($uvar as u16);
                } else if $uvar <= u32::MAX as u64 {
                    tag_with_u4!(L4_EXT_U32);
                    fixed!($uvar as u32);
                } else {
                    tag_with_u4!(L4_EXT_U64);
                    fixed!($uvar);
                }
            };
        }
        macro_rules! tag_with_ivar {
            ($ivar:expr) => {
                tag_with_uvar!(crate::util::zigzag_encode($ivar))
            };
        }
        macro_rules! tag_with_szvar {
            ($szvar:expr) => {{
                let uvar: u64 = $szvar.try_into().unwrap();
                tag_with_uvar!(uvar)
            }};
        }
        macro_rules! comptype {
            ($v:expr) => {
                $v.encode_to($buf)
            };
        }
        macro_rules! value {
            ($v:expr) => {
                $v.encode_to($buf)
            };
        }
        macro_rules! seq {
            ($s:expr) => {
                for v in $s {
                    value!(v);
                }
            };
        }
        macro_rules! seq_map {
            ($s:expr) => {
                for (k, v) in $s {
                    value!(k);
                    value!(v);
                }
            };
        }
    };
}

impl Type {
    pub fn encode_to(&self, buf: &mut Vec<u8>) {
        encode_macros!(self, buf);
        tag!();
        match self {
            Type::Unit |
            Type::Bool |
            Type::Int |
            Type::UInt |
            Type::Float |
            Type::String |
            Type::Bytes |
            Type::Type => {
            },

            Type::Option(t) |
            Type::List(t) => {
                comptype!(t)
            },

            Type::Map(tk, tv) => {
                comptype!(tk);
                comptype!(tv);
            },

            Type::Alias(ptr) |
            Type::Enum(ptr) |
            Type::Tuple(ptr) |
            Type::Struct(ptr) => {
                fixed!(ptr);
            },
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode_to(&mut buf);
        buf
    }
}

impl Value {
    pub fn encode_to(self, buf: &mut Vec<u8>) {
        encode_macros!(self, buf);
        match self {
            Value::Unit => {
                tag_with_noop!();

            },
            Value::Bool(b) => {
                tag_with_bool!(b);

            },
            Value::Int(i) => {
                tag_with_ivar!(i);

            },
            Value::UInt(u) => {
                tag_with_uvar!(u);

            },
            Value::Float(f) => {
                tag_with_noop!();
                fixed!(f);

            },
            Value::String(b) => {
                tag_with_szvar!(b.len());
                bytes!(b.as_bytes());

            },
            Value::Bytes(b) => {
                tag_with_szvar!(b.len());
                bytes!(b.as_slice());

            },
            Value::Option(t, opt) => {
                tag_with_bool!(opt.is_some());
                comptype!(t);
                if let Some(v) = *opt {
                    value!(v);
                }

            },
            Value::List(t, s) => {
                tag_with_szvar!(s.len());
                comptype!(t);
                seq!(s);

            },
            Value::Map((tk, tv), s) => {
                tag_with_szvar!(s.len());
                comptype!(tk);
                comptype!(tv);
                seq_map!(s);

            },
            Value::Alias(ptr, v) => {
                tag_with_noop!();
                fixed!(ptr);
                value!(v);

            },
            Value::Enum(ptr, ev, v) => {
                tag_with_uvar!(ev as u64);
                fixed!(ptr);
                value!(v);

            },
            Value::Tuple(ptr, s) => {
                tag_with_szvar!(s.len());
                fixed!(ptr);
                seq!(s);

            },
            Value::Struct(ptr, s) => {
                tag_with_szvar!(s.len());
                fixed!(ptr);
                seq!(s);
                
            },
            Value::Type(t) => {
                tag_with_noop!();
                comptype!(t);

            }
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode_to(&mut buf);
        buf
    }
}
