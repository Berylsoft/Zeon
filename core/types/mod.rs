#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypePtr {
    Std(StdPtr),
    Hash([u8; 7]),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StdPtr(u16);

pub fn check_stdptr(n: u16) -> bool {
    let h8 = n >> 8 as u8;
    h8 != 0xFF
}

impl StdPtr {
    pub fn from_u16(n: u16) -> StdPtr {
        assert!(check_stdptr(n));
        StdPtr(n)
    }

    #[inline]
    pub fn from_u16_unchecked(n: u16) -> StdPtr {
        StdPtr(n)
    }

    #[inline]
    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl TypePtr {
    pub fn from_u16(n: u16) -> TypePtr {
        assert!(check_stdptr(n));
        TypePtr::Std(StdPtr(n))
    }

    pub fn from_u16_unchecked(n: u16) -> TypePtr {
        TypePtr::Std(StdPtr(n))
    }

    pub fn from_path(path: &str) -> TypePtr {
        TypePtr::Hash(crate::util::shake256(path.as_bytes()))
    }

    pub fn as_std(self) -> Option<StdPtr> {
        match self {
            Self::Std(stdptr) => Some(stdptr),
            _ => None,
        }
    }

    pub fn as_std_inner(self) -> Option<u16> {
        match self {
            Self::Std(StdPtr(n)) => Some(n),
            _ => None,
        }
    }

    pub fn as_hash(self) -> Option<[u8; 7]> {
        match self {
            Self::Hash(hash) => Some(hash),
            _ => None,
        }
    }
}

pub type EnumVarient = u8;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, num_enum::TryFromPrimitive)]
pub enum Tag {
    Unit = 0x0,
    Bool,
    Int,
    UInt,
    Float,
    String,
    Bytes,

    Option,
    List,
    Map,

    Alias,
    Enum,
    Tuple,
    Struct,

    Type,
    ObjectRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Unit,
    Bool,
    Int,
    UInt,
    Float,
    String,
    Bytes,

    Option(Box<Type>),
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),

    Alias(TypePtr),
    Enum(TypePtr),
    Tuple(TypePtr),
    Struct(TypePtr),

    Type,
    ObjectRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Unit,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(u64),
    String(String),
    Bytes(Vec<u8>),

    Option(Type, Box<Option<Value>>),
    List(Type, Vec<Value>),
    Map((Type, Type), Vec<(Value, Value)>),

    Alias(TypePtr, Box<Value>),
    Enum(TypePtr, EnumVarient, Box<Value>),
    Tuple(TypePtr, Vec<Value>),
    Struct(TypePtr, Vec<Value>),

    Type(Type),
    ObjectRef(u16, u64),
}

mod casting;

pub(self) const L4_EXT_U8: u8 = 0xC;
pub(self) const L4_EXT_U16: u8 = 0xD;
pub(self) const L4_EXT_U32: u8 = 0xE;
pub(self) const L4_EXT_U64: u8 = 0xF;

macros::error_enum! {
    #[derive(Debug)]
    DecodeError {} convert {
        Io => std::io::Error,
        Utf8 => std::string::FromUtf8Error,
        Size => std::num::TryFromIntError,
        Tag => num_enum::TryFromPrimitiveError<Tag>,
    }
}

mod encode;
mod decode;

pub trait Schema {
    const PATH: &'static str;
    const PTR: TypePtr;
    fn schema() -> DefType;
    fn serialize(self) -> Value;
    fn deserialize(val: Value) -> Self;
}

pub enum DefType {
    Alias(Type),
    Enum(Vec<(String, Type)>),
    Tuple(Vec<Type>),
    Struct(Vec<(String, Type)>),
}

#[cfg(test)]
mod tests;
