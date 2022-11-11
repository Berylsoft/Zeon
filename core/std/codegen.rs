// This is a generated file. Do not modify, run `cargo run --bin schema-derive` to update.
#![allow(unused_imports, clippy::unit_arg, clippy::let_unit_value)]
pub mod types {
    use crate::{types::*, meta::ObjectRef};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Deftype {
        Alias(Type),
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
                    Self::Enum(_) => 1u8,
                    Self::Struct(_) => 2u8,
                },
                Box::new(
                    match self {
                        Self::Alias(val) => Value::Type(val),
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
                    Self::Enum(
                        val
                            .into_map()
                            .into_iter()
                            .map(|(sk, sv)| (sk.into_string(), sv.into_type()))
                            .collect(),
                    )
                }
                2u8 => {
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
        pub name: super::prim::SimpleName,
        pub val_type: Type,
    }
    impl Schema for TraitAttr {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(2u16);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(2u16),
                vec![
                    self.attr_type.serialize(), self.name.serialize(), Value::Type(self
                    .val_type),
                ],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [attr_type, name, val_type]: [Value; 3usize] = val
                .into_struct()
                .try_into()
                .unwrap();
            Self {
                attr_type: attr_type.deserialize_into(),
                name: name.deserialize_into(),
                val_type: val_type.into_type(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum TraitAttrType {
        Const(()),
        Mut(()),
        Iter(()),
        Iterset(()),
        Complex(()),
    }
    impl Schema for TraitAttrType {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(3u16);
        fn serialize(self) -> Value {
            Value::Enum(
                TypePtr::from_u16_unchecked(3u16),
                match &self {
                    Self::Const(_) => 0u8,
                    Self::Mut(_) => 1u8,
                    Self::Iter(_) => 2u8,
                    Self::Iterset(_) => 3u8,
                    Self::Complex(_) => 4u8,
                },
                Box::new(
                    match self {
                        Self::Const(val) => {
                            let _ = val;
                            Value::Unit
                        }
                        Self::Mut(val) => {
                            let _ = val;
                            Value::Unit
                        }
                        Self::Iter(val) => {
                            let _ = val;
                            Value::Unit
                        }
                        Self::Iterset(val) => {
                            let _ = val;
                            Value::Unit
                        }
                        Self::Complex(val) => {
                            let _ = val;
                            Value::Unit
                        }
                    },
                ),
            )
        }
        fn deserialize(val: Value) -> Self {
            let (variant, val) = val.into_enum();
            match variant {
                0u8 => Self::Const(val.into_unit()),
                1u8 => Self::Mut(val.into_unit()),
                2u8 => Self::Iter(val.into_unit()),
                3u8 => Self::Iterset(val.into_unit()),
                4u8 => Self::Complex(val.into_unit()),
                _ => unreachable!(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Trait {
        pub attrs: Vec<super::types::TraitAttr>,
        pub extends: Vec<super::meta::Typeptr>,
    }
    impl Schema for Trait {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(5u16);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(5u16),
                vec![
                    Value::List(Type::Enum(TypePtr::from_u16_unchecked(2u16)), self.attrs
                    .into_iter().map(| sv | sv.serialize()).collect()),
                    Value::List(Type::Enum(TypePtr::from_u16_unchecked(7u16)), self
                    .extends.into_iter().map(| sv | sv.serialize()).collect()),
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
                    .map(|sv| sv.deserialize_into())
                    .collect(),
            }
        }
    }
}
pub mod prim {
    use crate::{types::*, meta::ObjectRef};
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
}
pub mod pattern {
    use crate::{types::*, meta::ObjectRef};
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
    use crate::{types::*, meta::ObjectRef};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Typeptr {
        Std(super::meta::TypeptrStd),
        Hash(super::meta::TypeptrHash),
    }
    impl Schema for Typeptr {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(7u16);
        fn serialize(self) -> Value {
            Value::Enum(
                TypePtr::from_u16_unchecked(7u16),
                match &self {
                    Self::Std(_) => 0u8,
                    Self::Hash(_) => 1u8,
                },
                Box::new(
                    match self {
                        Self::Std(val) => val.serialize(),
                        Self::Hash(val) => val.serialize(),
                    },
                ),
            )
        }
        fn deserialize(val: Value) -> Self {
            let (variant, val) = val.into_enum();
            match variant {
                0u8 => Self::Std(val.deserialize_into()),
                1u8 => Self::Hash(val.deserialize_into()),
                _ => unreachable!(),
            }
        }
    }
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
}
