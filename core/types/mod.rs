use crate::meta::{Timestamp, ObjectPtr, TypePtr};

pub type EnumVariantId = u64;
pub type TupleItemId = u8;
pub type TraitAttrId = u8;

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
    CEnum,     // 0x0D
    Enum,      // 0x0E
    Struct,    // 0x0F
    Type,      // 0x10
    TypePtr,   // 0x11
    ObjectPtr, // 0x12
    Timestamp, // 0x13
    UInt8,     // 0x14
    UInt16,    // 0x15
    UInt32,    // 0x16
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
    CEnum,     // 0x9
    Enum,      // 0xA
    Struct,    // 0xB
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
    ObjectPtr, // 0x8
    Timestamp, // 0x9
    UInt8,     // 0xA
    UInt16,    // 0xB
    UInt32,    // 0xC
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
    CEnum(TypePtr),
    Enum(TypePtr),
    Struct(TypePtr),

    Type,
    TypePtr,
    ObjectPtr,
    Timestamp,

    UInt8,
    UInt16,
    UInt32,
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
    CEnum(TypePtr, EnumVariantId),
    Enum(TypePtr, EnumVariantId, Box<Value>),
    Struct(TypePtr, Vec<Value>),

    Type(Type),
    TypePtr(TypePtr),
    ObjectPtr(ObjectPtr),
    Timestamp(Timestamp),

    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
}

mod casting;

pub(self) const EXT8: u8 = 0xC;
pub(self) const EXT16: u8 = 0xD;
pub(self) const EXT32: u8 = 0xE;
pub(self) const EXT64: u8 = 0xF;

foundations::error_enum! {
    #[derive(Debug)]
    pub enum DecodeError {
        FloatL4(u8),
        TooShort((usize, usize)),
        TooLong(usize),
    } convert {
        Utf8 => std::string::FromUtf8Error,
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

pub use crate::std::codegen::types::{DefType, Trait, CommitAttr, CommitAttrType, StateAttr, Validator};

#[cfg(test)]
mod tests;
