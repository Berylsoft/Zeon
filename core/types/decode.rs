use foundations::{num_compress::*, usize_casting::*, bytes_read::*};
use super::*;

type Result<T> = DecodeResult<T>;

struct Reader<'a> {
    bytes: &'a [u8],
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Reader<'a> {
        Reader { bytes }
    }

    fn finish(self) -> Result<()> {
        if self.bytes.is_empty() {
            Ok(())
        } else {
            Err(DecodeError::TooLong(self.bytes.len()))
        }
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.bytes.read(buf).map_err(DecodeError::TooShort)
    }

    #[inline]
    fn bytes(&mut self, sz: usize) -> Result<Vec<u8>> {
        self.bytes.read_to_vec(sz).map_err(DecodeError::TooShort)
    }

    #[inline]
    fn bytes_sized<const N: usize>(&mut self) -> Result<[u8; N]> {
        self.bytes.read_to_array().map_err(DecodeError::TooShort)
    }

    fn u8(&mut self) -> Result<u8> {
        self.bytes.read_byte().ok_or(DecodeError::TooShort((0, 1)))
    }

    fn u16(&mut self) -> Result<u16> {
        self.bytes_sized().map(u16::from_be_bytes)
    }

    fn u32(&mut self) -> Result<u32> {
        self.bytes_sized().map(u32::from_be_bytes)
    }

    fn u64(&mut self) -> Result<u64> {
        self.bytes_sized().map(u64::from_be_bytes)
    }

    fn i64(&mut self) -> Result<i64> {
        self.bytes_sized().map(i64::from_be_bytes)
    }

    fn typeptr(&mut self) -> Result<TypePtr> {
        let h8 = self.u8()?;
        Ok(match h8 {
            0xFF => {
                let hash = self.bytes_sized()?;
                TypePtr::Hash(hash)
            },
            h8 => {
                let l8 = self.u8()?;
                TypePtr::from_u16_unchecked(u16::from_be_bytes([h8, l8]))
            },
        })
    }

    fn ty(&mut self) -> Result<Type> {
        let tag = self.u8()?.try_into()?;
        Ok(match tag {
            Tag::Unknown => Type::Unknown,
            Tag::Unit => Type::Unit,
            Tag::Bool => Type::Bool,
            Tag::Int => Type::Int,
            Tag::UInt => Type::UInt,
            Tag::Float => Type::Float,
            Tag::String => Type::String,
            Tag::Bytes => Type::Bytes,
            Tag::Type => Type::Type,
            Tag::TypePtr => Type::TypePtr,
            Tag::ObjectPtr => Type::ObjectPtr,
            Tag::Timestamp => Type::Timestamp,
            Tag::UInt8 => Type::UInt8,
            Tag::UInt16 => Type::UInt16,
            Tag::UInt32 => Type::UInt32,

            Tag::Option => {
                let t = self.ty()?;
                Type::Option(Box::new(t))
            },
            Tag::List => {
                let t = self.ty()?;
                Type::List(Box::new(t))
            },
            Tag::Map => {
                let tk = self.ty()?;
                let tv = self.ty()?;
                Type::Map(Box::new(tk), Box::new(tv))
            },

            Tag::Tuple => {
                let len = self.u8()? as usize;
                let mut s = Vec::with_capacity(len);
                for _ in 0..len {
                    let t = self.ty()?;
                    s.push(t)
                }
                Type::Tuple(s)
            },

            Tag::Alias => {
                let ptr = self.typeptr()?;
                Type::Alias(ptr)
            },
            Tag::CEnum => {
                let ptr = self.typeptr()?;
                Type::CEnum(ptr)
            },
            Tag::Enum => {
                let ptr = self.typeptr()?;
                Type::Enum(ptr)
            },
            Tag::Struct => {
                let ptr = self.typeptr()?;
                Type::Struct(ptr)
            },
        })
    }

    fn with_ltag(&mut self, l4: u8) -> Result<LTag> {
        Ok(l4.try_into()?)
    }

    fn with_uvar(&mut self, l4: u8) -> Result<u64> {
        Ok(match l4 {
            EXT8 => self.u8()? as u64,
            EXT16 => self.u16()? as u64,
            EXT32 => self.u32()? as u64,
            EXT64 => self.u64()?,
            s => s as u64,
        })
    }

    fn with_ivar(&mut self, l4: u8) -> Result<i64> {
        Ok(zigzag_decode(self.with_uvar(l4)?))
    }

    fn with_szvar(&mut self, l4: u8) -> Result<usize> {
        Ok(u64_usize(self.with_uvar(l4)?))
    }

    fn with_fvar(&mut self, l4: u8) -> Result<u64> {
        if l4 > 8 { return Err(DecodeError::FloatL4(l4)); }
        let mut buf = [0; 8];
        self.read_exact(&mut buf[0..l4 as usize])?;
        Ok(u64::from_be_bytes(buf))
    }

    fn val_seq(&mut self, size: usize) -> Result<Vec<Value>> {
        let mut s = Vec::with_capacity(size);
        for _ in 0..size {
            let v = self.val()?;
            s.push(v)
        }
        Ok(s)
    }

    fn val_seq_map(&mut self, size: usize) -> Result<Vec<(Value, Value)>> {
        let mut s = Vec::with_capacity(size);
        for _ in 0..size {
            let k = self.val()?;
            let v = self.val()?;
            s.push((k, v))
        }
        Ok(s)
    }

    fn val(&mut self) -> Result<Value> {
        let (htag, l4) = to_h4l4(self.u8()?);
        Ok(match htag.try_into()? {
            HTag::L4 => {
                let ltag = self.with_ltag(l4)?;
                match ltag {
                    LTag::Unit => Value::Unit,
                    LTag::True => Value::Bool(true),
                    LTag::False => Value::Bool(false),
                    opt @ (LTag::None | LTag::Some) => {
                        let t = self.ty()?;
                        let opt = match opt {
                            LTag::None => None,
                            LTag::Some => Some(self.val()?),
                            _ => unreachable!(), // waiting for flow-sensitive typing implemented
                        };
                        Value::Option(t, Box::new(opt))
                    },
                    LTag::Alias => {
                        let ptr = self.typeptr()?;
                        let v = self.val()?;
                        Value::Alias(ptr, Box::new(v))
                    },
                    LTag::Type => {
                        let t = self.ty()?;
                        Value::Type(t)
                    },
                    LTag::TypePtr => {
                        let ptr = self.typeptr()?;
                        Value::TypePtr(ptr)
                    },
                    LTag::ObjectPtr => {
                        let ot = self.u16()?;
                        let oid = self.u64()?;
                        Value::ObjectPtr(ObjectPtr { ot, oid })
                    },
                    LTag::Timestamp => {
                        let secs = self.i64()?;
                        let nanos = self.u32()?;
                        Value::Timestamp(Timestamp { secs, nanos })
                    },
                    LTag::UInt8 => {
                        let u = self.u8()?;
                        Value::UInt8(u)
                    },
                    LTag::UInt16 => {
                        let u = self.u16()?;
                        Value::UInt16(u)
                    },
                    LTag::UInt32 => {
                        let u = self.u32()?;
                        Value::UInt32(u)
                    },
                }
            },
            HTag::Int => {
                let i = self.with_ivar(l4)?;
                Value::Int(i)
            },
            HTag::UInt => {
                let u = self.with_uvar(l4)?;
                Value::UInt(u)
            },
            HTag::Float => {
                let f = self.with_fvar(l4)?;
                Value::Float(f)
            },
            HTag::String => {
                let len = self.with_szvar(l4)?;
                let b = self.bytes(len)?;
                Value::String(String::from_utf8(b)?)
            },
            HTag::Bytes => {
                let len = self.with_szvar(l4)?;
                let b = self.bytes(len)?;
                Value::Bytes(b)
            },
            HTag::List => {
                let len = self.with_szvar(l4)?;
                let t = self.ty()?;
                let s = self.val_seq(len)?;
                Value::List(t, s)
            },
            HTag::Map => {
                let len = self.with_szvar(l4)?;
                let tk = self.ty()?;
                let tv = self.ty()?;
                let s = self.val_seq_map(len)?;
                Value::Map((tk, tv), s)
            },
            HTag::Tuple => {
                let len = self.with_szvar(l4)?;
                let s = self.val_seq(len)?;
                Value::Tuple(s)
            },
            HTag::CEnum => {
                let ev = self.with_uvar(l4)?;
                let ptr = self.typeptr()?;
                Value::CEnum(ptr, ev)
            },
            HTag::Enum => {
                let ev = self.with_uvar(l4)?;
                let ptr = self.typeptr()?;
                let v = self.val()?;
                Value::Enum(ptr, ev, Box::new(v))
            },
            HTag::Struct => {
                let len = self.with_szvar(l4)?;
                let ptr = self.typeptr()?;
                let s = self.val_seq(len)?;
                Value::Struct(ptr, s)
            },
        })
    }
}

impl Value {
    pub fn decode(buf: &[u8]) -> Result<Value> {
        let mut reader = Reader::new(buf);
        let val = reader.val()?;
        reader.finish()?;
        Ok(val)
    }
}
