use crate::meta::{ObjectRef, Timestamp};

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
    Unknown     = 0x00,
    Unit,      // 0x01
    Bool,      // 0x02
    Int,       // 0x03
    UInt,      // 0x04
    Float,     // 0x05
    String,    // 0x06
    Bytes,     // 0x07
    Option,    // 0x08
    List,      // 0x09
    Map,       // 0x0A
    Tuple,     // 0x0B
    Alias,     // 0x0C
    Enum,      // 0x0D
    Struct,    // 0x0E
    Type,      // 0x0F
    TypePtr,   // 0x10
    ObjectRef, // 0x11
    Timestamp, // 0x12
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, num_enum::TryFromPrimitive)]
pub enum HTag {
    L4          = 0x0,
    Int,       // 0x1
    UInt,      // 0x2
    Float,     // 0x3
    String,    // 0x4
    Bytes,     // 0x5
    List,      // 0x6
    Map,       // 0x7
    Tuple,     // 0x8
    Enum,      // 0x9
    Struct,    // 0xA
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, num_enum::TryFromPrimitive)]
pub enum LTag {
    Unit        = 0x0,
    False,     // 0x1
    True,      // 0x2
    None,      // 0x3
    Some,      // 0x4
    Alias,     // 0x5
    Type,      // 0x6
    TypePtr,   // 0x7
    ObjectRef, // 0x8
    Timestamp, // 0x9
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Unknown,

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
    TypePtr,
    ObjectRef,
    Timestamp,
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
    TypePtr(TypePtr),
    ObjectRef(ObjectRef),
    Timestamp(Timestamp),
}

mod casting;

pub(self) const EXT8: u8 = 0xC;
pub(self) const EXT16: u8 = 0xD;
pub(self) const EXT32: u8 = 0xE;
pub(self) const EXT64: u8 = 0xF;

macros::error_enum! {
    #[derive(Debug)]
    DecodeError {} convert {
        Io => std::io::Error,
        Utf8 => std::string::FromUtf8Error,
        Size => std::num::TryFromIntError,
        Tag => num_enum::TryFromPrimitiveError<Tag>,
        HTag => num_enum::TryFromPrimitiveError<HTag>,
        LTag => num_enum::TryFromPrimitiveError<LTag>,
    }
}

pub type DecodeResult<T> = Result<T, DecodeError>;

mod encode;
mod decode;

pub trait Schema {
    const PTR: TypePtr;
    fn serialize(self) -> Value;
    fn deserialize(val: Value) -> Self;
}

pub use crate::std::codegen::types::Deftype as DefType;
pub mod traits {
    pub use crate::std::codegen::{prim::SimpleName, types::{Trait, TraitAttr, TraitAttrType}};
}

#[cfg(test)]
mod tests;
