use crate::meta::ObjectRef;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypePtr {
    Std(StdPtr),
    Hash([u8; 7]),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StdPtr(u16);

pub type EnumVariant = u8;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, num_enum::TryFromPrimitive)]
pub enum Tag {
    Unit       = 0x0,
    Bool,      // 0x1
    Int,       // 0x2
    UInt,      // 0x3
    Float,     // 0x4
    String,    // 0x5
    Bytes,     // 0x6

    Option,    // 0x7
    List,      // 0x8
    Map,       // 0x9

    Tuple,     // 0xA

    Alias,     // 0xB
    Enum,      // 0xC
    Struct,    // 0xD

    Type,      // 0xE
    ObjectRef, // 0xF
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

    Tuple(Vec<Type>),

    Alias(TypePtr),
    Enum(TypePtr),
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

    Tuple(Vec<Value>),

    Alias(TypePtr, Box<Value>),
    Enum(TypePtr, EnumVariant, Box<Value>),
    Struct(TypePtr, Vec<Value>),

    Type(Type),
    ObjectRef(ObjectRef),
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
    const PTR: TypePtr;
    fn serialize(self) -> Value;
    fn deserialize(val: Value) -> Self;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DefType {
    Alias(Type),
    Enum(Vec<(String, Type)>),
    Struct(Vec<(String, Type)>),
}

#[cfg(test)]
mod tests;
