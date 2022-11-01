pub type TypePtr = u64;
pub type EnumVarient = u8;

#[repr(u8)]
#[derive(Clone, Copy, num_enum::TryFromPrimitive)]
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
    Set,
    Map,

    Alias,
    Enum,
    Tuple,
    Struct,

    Type,
}

#[derive(Clone, PartialEq, Eq)]
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
    Set(Box<Type>),
    Map(Box<Type>, Box<Type>),

    Alias(TypePtr),
    Enum(TypePtr),
    Tuple(TypePtr),
    Struct(TypePtr),

    Type,
}

#[derive(Clone)]
pub enum Value {
    Unit,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    // with byte length
    String(String),
    // with byte length
    Bytes(Vec<u8>),

    Option(Type, Box<Option<Value>>),
    // with seq length
    List(Type, Vec<Value>),
    // with seq length
    Set(Type, Vec<Value>),
    // with seq length
    Map((Type, Type), Vec<(Value, Value)>),

    Alias(TypePtr, Box<Value>),
    // varient name & type provided by type
    Enum(TypePtr, EnumVarient, Box<Value>),
    // seq length provided by type
    Tuple(TypePtr, Vec<Value>),
    // seq length & field name provided by type
    Struct(TypePtr, Vec<Value>),

    Type(Type),
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

pub trait DefTypeSchema {
    const PATH: &'static str;
    fn deftype() -> DefType;
    fn from_value(v: Value) -> Self;
    fn to_value(self) -> Value;

    fn ptr() -> TypePtr {
        crate::util::shake256_u64(Self::PATH.as_bytes())
    }
}

pub enum DefType {
    Alias(Type),
    Enum(Vec<(String, Type)>),
    Tuple(Vec<Type>),
    Struct(Vec<(String, Type)>),
}
