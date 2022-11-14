// This is a generated file. Do not modify, run `cargo run --bin schema-derive` to update.
#![allow(
    unused_imports,
    clippy::unit_arg,
    clippy::let_unit_value,
    clippy::redundant_closure,
)]
pub mod types {
    use crate::{types::*, meta::{ObjectRef, Timestamp}};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Deftype {
        Alias(Type),
        CEnum(Vec<String>),
        Enum(Vec<(String, Type)>),
        Struct(Vec<(String, Type)>),
    }
    impl Schema for Deftype {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(0u16);
        fn serialize(self) -> Value {
            Value::Enum(
                TypePtr::from_u16_unchecked(0u16),
                match &self {
                    Self::Alias(_) => 0u8,
                    Self::CEnum(_) => 1u8,
                    Self::Enum(_) => 2u8,
                    Self::Struct(_) => 3u8,
                },
                Box::new(
                    match self {
                        Self::Alias(val) => Value::Type(val),
                        Self::CEnum(val) => {
                            Value::List(
                                Type::String,
                                val.into_iter().map(|sv| Value::String(sv)).collect(),
                            )
                        }
                        Self::Enum(val) => {
                            Value::Map(
                                (Type::String, Type::Type),
                                val
                                    .into_iter()
                                    .map(|(sk, sv)| (Value::String(sk), Value::Type(sv)))
                                    .collect(),
                            )
                        }
                        Self::Struct(val) => {
                            Value::Map(
                                (Type::String, Type::Type),
                                val
                                    .into_iter()
                                    .map(|(sk, sv)| (Value::String(sk), Value::Type(sv)))
                                    .collect(),
                            )
                        }
                    },
                ),
            )
        }
        fn deserialize(val: Value) -> Self {
            let (variant, val) = val.into_enum();
            match variant {
                0u8 => Self::Alias(val.into_type()),
                1u8 => {
                    Self::CEnum(
                        val.into_list().into_iter().map(|sv| sv.into_string()).collect(),
                    )
                }
                2u8 => {
                    Self::Enum(
                        val
                            .into_map()
                            .into_iter()
                            .map(|(sk, sv)| (sk.into_string(), sv.into_type()))
                            .collect(),
                    )
                }
                3u8 => {
                    Self::Struct(
                        val
                            .into_map()
                            .into_iter()
                            .map(|(sk, sv)| (sk.into_string(), sv.into_type()))
                            .collect(),
                    )
                }
                _ => unreachable!(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct TraitAttr {
        pub attr_type: super::types::TraitAttrType,
        pub attr_name: String,
        pub val_type: Type,
    }
    impl Schema for TraitAttr {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(2u16);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(2u16),
                vec![
                    self.attr_type.serialize(), Value::String(self.attr_name),
                    Value::Type(self.val_type),
                ],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [attr_type, attr_name, val_type]: [Value; 3usize] = val
                .into_struct()
                .try_into()
                .unwrap();
            Self {
                attr_type: attr_type.deserialize_into(),
                attr_name: attr_name.into_string(),
                val_type: val_type.into_type(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum TraitAttrType {
        Const,
        Mut,
        Iter,
        Iterset,
        Complex,
    }
    impl Schema for TraitAttrType {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(3u16);
        fn serialize(self) -> Value {
            Value::CEnum(
                TypePtr::from_u16_unchecked(3u16),
                match &self {
                    Self::Const => 0u8,
                    Self::Mut => 1u8,
                    Self::Iter => 2u8,
                    Self::Iterset => 3u8,
                    Self::Complex => 4u8,
                },
            )
        }
        fn deserialize(val: Value) -> Self {
            let variant = val.into_c_enum();
            match variant {
                0u8 => Self::Const,
                1u8 => Self::Mut,
                2u8 => Self::Iter,
                3u8 => Self::Iterset,
                4u8 => Self::Complex,
                _ => unreachable!(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Trait {
        pub attrs: Vec<super::types::TraitAttr>,
        pub extends: Vec<TypePtr>,
    }
    impl Schema for Trait {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(5u16);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(5u16),
                vec![
                    Value::List(Type::Enum(TypePtr::from_u16_unchecked(2u16)), self.attrs
                    .into_iter().map(| sv | sv.serialize()).collect()),
                    Value::List(Type::TypePtr, self.extends.into_iter().map(| sv |
                    Value::TypePtr(sv)).collect()),
                ],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [attrs, extends]: [Value; 2usize] = val
                .into_struct()
                .try_into()
                .unwrap();
            Self {
                attrs: attrs
                    .into_list()
                    .into_iter()
                    .map(|sv| sv.deserialize_into())
                    .collect(),
                extends: extends
                    .into_list()
                    .into_iter()
                    .map(|sv| sv.into_typeptr())
                    .collect(),
            }
        }
    }
}
pub mod prim {
    use crate::{types::*, meta::{ObjectRef, Timestamp}};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UnixTs(pub u64);
    impl Schema for UnixTs {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(1u16);
        fn serialize(self) -> Value {
            Value::UInt(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_uint())
        }
    }
    impl From<u64> for UnixTs {
        fn from(val: u64) -> Self {
            Self(val)
        }
    }
    impl From<UnixTs> for u64 {
        fn from(val: UnixTs) -> u64 {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SimpleName(pub String);
    impl Schema for SimpleName {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(4u16);
        fn serialize(self) -> Value {
            Value::String(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_string())
        }
    }
    impl From<String> for SimpleName {
        fn from(val: String) -> Self {
            Self(val)
        }
    }
    impl From<SimpleName> for String {
        fn from(val: SimpleName) -> String {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct U8(pub u64);
    impl Schema for U8 {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(8u16);
        fn serialize(self) -> Value {
            Value::UInt(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_uint())
        }
    }
    impl From<u64> for U8 {
        fn from(val: u64) -> Self {
            Self(val)
        }
    }
    impl From<U8> for u64 {
        fn from(val: U8) -> u64 {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct U16(pub u64);
    impl Schema for U16 {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(9u16);
        fn serialize(self) -> Value {
            Value::UInt(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_uint())
        }
    }
    impl From<u64> for U16 {
        fn from(val: u64) -> Self {
            Self(val)
        }
    }
    impl From<U16> for u64 {
        fn from(val: U16) -> u64 {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct U32(pub u64);
    impl Schema for U32 {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(10u16);
        fn serialize(self) -> Value {
            Value::UInt(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_uint())
        }
    }
    impl From<u64> for U32 {
        fn from(val: u64) -> Self {
            Self(val)
        }
    }
    impl From<U32> for u64 {
        fn from(val: U32) -> u64 {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct I8(pub i64);
    impl Schema for I8 {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(11u16);
        fn serialize(self) -> Value {
            Value::Int(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_int())
        }
    }
    impl From<i64> for I8 {
        fn from(val: i64) -> Self {
            Self(val)
        }
    }
    impl From<I8> for i64 {
        fn from(val: I8) -> i64 {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct I16(pub i64);
    impl Schema for I16 {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(12u16);
        fn serialize(self) -> Value {
            Value::Int(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_int())
        }
    }
    impl From<i64> for I16 {
        fn from(val: i64) -> Self {
            Self(val)
        }
    }
    impl From<I16> for i64 {
        fn from(val: I16) -> i64 {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct I32(pub i64);
    impl Schema for I32 {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(13u16);
        fn serialize(self) -> Value {
            Value::Int(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_int())
        }
    }
    impl From<i64> for I32 {
        fn from(val: i64) -> Self {
            Self(val)
        }
    }
    impl From<I32> for i64 {
        fn from(val: I32) -> i64 {
            val.0
        }
    }
}
pub mod pattern {
    use crate::{types::*, meta::{ObjectRef, Timestamp}};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum RefsetItem {
        Remove(ObjectRef),
        Add(ObjectRef),
    }
    impl Schema for RefsetItem {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(6u16);
        fn serialize(self) -> Value {
            Value::Enum(
                TypePtr::from_u16_unchecked(6u16),
                match &self {
                    Self::Remove(_) => 0u8,
                    Self::Add(_) => 1u8,
                },
                Box::new(
                    match self {
                        Self::Remove(val) => Value::ObjectRef(val),
                        Self::Add(val) => Value::ObjectRef(val),
                    },
                ),
            )
        }
        fn deserialize(val: Value) -> Self {
            let (variant, val) = val.into_enum();
            match variant {
                0u8 => Self::Remove(val.into_objectref()),
                1u8 => Self::Add(val.into_objectref()),
                _ => unreachable!(),
            }
        }
    }
}
pub mod meta {
    use crate::{types::*, meta::{ObjectRef, Timestamp}};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct TypeptrStd(pub u64);
    impl Schema for TypeptrStd {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(14u16);
        fn serialize(self) -> Value {
            Value::UInt(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_uint())
        }
    }
    impl From<u64> for TypeptrStd {
        fn from(val: u64) -> Self {
            Self(val)
        }
    }
    impl From<TypeptrStd> for u64 {
        fn from(val: TypeptrStd) -> u64 {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct TypeptrHash(pub u64);
    impl Schema for TypeptrHash {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(15u16);
        fn serialize(self) -> Value {
            Value::UInt(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_uint())
        }
    }
    impl From<u64> for TypeptrHash {
        fn from(val: u64) -> Self {
            Self(val)
        }
    }
    impl From<TypeptrHash> for u64 {
        fn from(val: TypeptrHash) -> u64 {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ObjectType(pub u64);
    impl Schema for ObjectType {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(16u16);
        fn serialize(self) -> Value {
            Value::UInt(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_uint())
        }
    }
    impl From<u64> for ObjectType {
        fn from(val: u64) -> Self {
            Self(val)
        }
    }
    impl From<ObjectType> for u64 {
        fn from(val: ObjectType) -> u64 {
            val.0
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ObjectId(pub u64);
    impl Schema for ObjectId {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(17u16);
        fn serialize(self) -> Value {
            Value::UInt(self.0)
        }
        fn deserialize(val: Value) -> Self {
            Self(val.into_uint())
        }
    }
    impl From<u64> for ObjectId {
        fn from(val: u64) -> Self {
            Self(val)
        }
    }
    impl From<ObjectId> for u64 {
        fn from(val: ObjectId) -> u64 {
            val.0
        }
    }
}
