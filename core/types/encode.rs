use super::*;

struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn new() -> Writer {
        Writer { bytes: Vec::new() }
    }

    fn into_bytes(self) -> Vec<u8> {
        let Writer { bytes } = self;
        bytes
    }

    #[inline]
    fn bytes<B: AsRef<[u8]>>(&mut self, bytes: B) {
        self.bytes.extend_from_slice(bytes.as_ref());
    }

    // #[inline]
    // fn bytes_sized<const N: usize>(&mut self, bytes: [u8; N]) {
    //     self.bytes.extend_from_slice(bytes.as_slice());
    // }

    #[inline]
    fn u8(&mut self, n: u8) {
        self.bytes.push(n);
    }

    fn u16(&mut self, n: u16) {
        self.bytes(n.to_be_bytes());
    }

    fn u32(&mut self, n: u32) {
        self.bytes(n.to_be_bytes());
    }

    fn u64(&mut self, n: u64) {
        self.bytes(n.to_be_bytes());
    }

    fn i64(&mut self, n: i64) {
        self.bytes(n.to_be_bytes());
    }

    fn typeptr(&mut self, ptr: &TypePtr) {
        match ptr {
            TypePtr::Std(stdptr) => {
                self.u16(stdptr.to_u16());
            },
            TypePtr::Hash(hash) => {
                self.u8(0xFF);
                self.bytes(hash);
            }
        }
    }

    fn ty(&mut self, t: &Type) {
        self.u8(t.as_tag() as u8);
        match t {
            Type::Unknown |
            Type::Unit |
            Type::Bool |
            Type::Int |
            Type::UInt |
            Type::Float |
            Type::String |
            Type::Bytes |
            Type::Type |
            Type::TypePtr |
            Type::ObjectRef |
            Type::Timestamp => {},

            Type::Option(t) |
            Type::List(t) => {
                self.ty(t);

            },
            Type::Map(tk, tv) => {
                self.ty(tk);
                self.ty(tv);

            },

            Type::Tuple(s) => {
                self.u8(s.len().try_into().unwrap());

                for t in s {
                    self.ty(t);
                }

            }

            Type::Alias(ptr) |
            Type::Enum(ptr) |
            Type::Struct(ptr) => {
                self.typeptr(ptr);

            },
        }
    }

    #[inline]
    fn with_l4(&mut self, htag: HTag, l: u8) {
        self.u8(crate::util::from_h4l4(htag as u8, l));
    }

    fn with_ltag(&mut self, htag: HTag, l: LTag) {
        self.with_l4(htag, l as u8)
    }

    fn with_uvar(&mut self, htag: HTag, u: u64) {
        if u < EXT8 as u64 {
            self.with_l4(htag, u as u8);
        } else if u <= (u8::MAX as u64) {
            self.with_l4(htag, EXT8);
            self.u8(u as u8);
        } else if u <= (u16::MAX as u64) {
            self.with_l4(htag, EXT16);
            self.u16(u as u16);
        } else if u <= (u32::MAX as u64) {
            self.with_l4(htag, EXT32);
            self.u32(u as u32);
        } else {
            self.with_l4(htag, EXT64);
            self.u64(u);
        }
    }

    fn with_ivar(&mut self, htag: HTag, i: i64) {
        self.with_uvar(htag, crate::util::zigzag_encode(i))
    }

    fn with_szvar(&mut self, htag: HTag, sz: usize) {
        self.with_uvar(htag, sz.try_into().unwrap())
    }

    fn with_fvar(&mut self, htag: HTag, f: u64) {
        let len = crate::util::float_find_zero(f);
        self.with_l4(htag, len as u8);
        let buf = f.to_be_bytes();
        self.bytes(&buf[0..len]);
    }

    fn val_seq(&mut self, s: &Vec<Value>) {

        for v in s {
            self.val(v);
        }

    }

    fn val_seq_map(&mut self, s: &Vec<(Value, Value)>) {

        for (k, v) in s {
            self.val(k);
            self.val(v);
        }

    }

    fn val(&mut self, val: &Value) {
        let htag = val.as_htag();
        match val {
            Value::Unit => {
                self.with_ltag(htag, LTag::Unit);

            },
            Value::Bool(b) => {
                if *b {
                    self.with_ltag(htag, LTag::True);
                } else {
                    self.with_ltag(htag, LTag::False);
                }

            },
            Value::Int(i) => {
                self.with_ivar(htag, *i);

            },
            Value::UInt(u) => {
                self.with_uvar(htag, *u);

            },
            Value::Float(f) => {
                self.with_fvar(htag, *f);

            },
            Value::String(b) => {
                self.with_szvar(htag, b.len());
                self.bytes(b);

            },
            Value::Bytes(b) => {
                self.with_szvar(htag, b.len());
                self.bytes(b);

            },
            Value::Option(t, opt) => {
                if let Some(v) = opt.as_ref() {
                    self.with_ltag(htag, LTag::Some);
                    self.ty(t);
                    self.val(v);
                } else {
                    self.with_ltag(htag, LTag::None);
                    self.ty(t);
                }

            },
            Value::List(t, s) => {
                self.with_szvar(htag, s.len());
                self.ty(t);
                self.val_seq(s);

            },
            Value::Map((tk, tv), s) => {
                self.with_szvar(htag, s.len());
                self.ty(tk);
                self.ty(tv);
                self.val_seq_map(s);

            },
            Value::Tuple(s) => {
                self.with_szvar(htag, s.len());
                self.val_seq(s);

            },
            Value::Alias(ptr, v) => {
                self.with_ltag(htag, LTag::Alias);
                self.typeptr(ptr);
                self.val(v);

            },
            Value::Enum(ptr, ev, v) => {
                self.with_uvar(htag, *ev as u64);
                self.typeptr(ptr);
                self.val(v);

            },
            Value::Struct(ptr, s) => {
                self.with_szvar(htag, s.len());
                self.typeptr(ptr);
                self.val_seq(s);
                
            },
            Value::Type(t) => {
                self.with_ltag(htag, LTag::Type);
                self.ty(t);

            }
            Value::TypePtr(ptr) => {
                self.with_ltag(htag, LTag::TypePtr);
                self.typeptr(ptr);

            }
            Value::ObjectRef(ObjectRef { ot, oid }) => {
                self.with_ltag(htag, LTag::ObjectRef);
                self.u16(*ot);
                self.u64(*oid);

            }
            Value::Timestamp(Timestamp { secs, nanos }) => {
                self.with_ltag(htag, LTag::Timestamp);
                self.i64(*secs);
                self.u32(*nanos);

            }
        }
    }
}

impl Value {
    pub fn encode(&self) -> Vec<u8> {
        let mut writer = Writer::new();
        writer.val(self);
        writer.into_bytes()
    }
}
