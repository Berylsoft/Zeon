// This is a generated file. Do not modify, run `cargo run --bin schema-derive` to update.
#![allow(
    unused_imports,
    clippy::unit_arg,
    clippy::let_unit_value,
    clippy::redundant_closure,
    clippy::redundant_field_names,
    clippy::map_identity,
)]
pub mod types {
    use crate::{types::*, meta::{Timestamp, ObjectPtr, TypePtr}};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum DefType {
        Alias(Type),
        CEnum(Vec<String>),
        Enum(Vec<(String, Type)>),
        Struct(Vec<(String, Type)>),
    }
    impl Schema for DefType {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(0);
        fn serialize(self) -> Value {
            Value::Enum(
                TypePtr::from_u16_unchecked(0),
                match &self {
                    Self::Alias(_) => 0,
                    Self::CEnum(_) => 1,
                    Self::Enum(_) => 2,
                    Self::Struct(_) => 3,
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
                0 => Self::Alias(val.into_type()),
                1 => {
                    Self::CEnum(
                        val.into_list().into_iter().map(|sv| sv.into_string()).collect(),
                    )
                }
                2 => {
                    Self::Enum(
                        val
                            .into_map()
                            .into_iter()
                            .map(|(sk, sv)| (sk.into_string(), sv.into_type()))
                            .collect(),
                    )
                }
                3 => {
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
    pub struct CommitAttr {
        pub attr_type: super::types::CommitAttrType,
        pub attr_name: String,
        pub val_type: Type,
    }
    impl Schema for CommitAttr {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(2);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(2),
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CommitAttrType {
        Const,
        Mut,
        IterList,
        IterSet,
        Complex,
    }
    impl Schema for CommitAttrType {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(3);
        fn serialize(self) -> Value {
            Value::CEnum(
                TypePtr::from_u16_unchecked(3),
                match &self {
                    Self::Const => 0,
                    Self::Mut => 1,
                    Self::IterList => 2,
                    Self::IterSet => 3,
                    Self::Complex => 4,
                },
            )
        }
        fn deserialize(val: Value) -> Self {
            let variant = val.into_c_enum();
            match variant {
                0 => Self::Const,
                1 => Self::Mut,
                2 => Self::IterList,
                3 => Self::IterSet,
                4 => Self::Complex,
                _ => unreachable!(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Trait {
        pub commit_attrs: Vec<super::types::CommitAttr>,
        pub state_attrs: Vec<super::types::StateAttr>,
        pub extends: Vec<TypePtr>,
        pub validators: Vec<super::types::Validator>,
    }
    impl Schema for Trait {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(5);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(5),
                vec![
                    Value::List(Type::Enum(TypePtr::from_u16_unchecked(2)), self
                    .commit_attrs.into_iter().map(| sv | sv.serialize()).collect()),
                    Value::List(Type::Enum(TypePtr::from_u16_unchecked(10)), self
                    .state_attrs.into_iter().map(| sv | sv.serialize()).collect()),
                    Value::List(Type::TypePtr, self.extends.into_iter().map(| sv |
                    Value::TypePtr(sv)).collect()),
                    Value::List(Type::Struct(TypePtr::from_u16_unchecked(11)), self
                    .validators.into_iter().map(| sv | sv.serialize()).collect()),
                ],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [commit_attrs, state_attrs, extends, validators]: [Value; 4usize] = val
                .into_struct()
                .try_into()
                .unwrap();
            Self {
                commit_attrs: commit_attrs
                    .into_list()
                    .into_iter()
                    .map(|sv| sv.deserialize_into())
                    .collect(),
                state_attrs: state_attrs
                    .into_list()
                    .into_iter()
                    .map(|sv| sv.deserialize_into())
                    .collect(),
                extends: extends
                    .into_list()
                    .into_iter()
                    .map(|sv| sv.into_type_ptr())
                    .collect(),
                validators: validators
                    .into_list()
                    .into_iter()
                    .map(|sv| sv.deserialize_into())
                    .collect(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct StateAttr {
        pub attr_name: String,
        pub val_type: Type,
    }
    impl Schema for StateAttr {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(10);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(10),
                vec![Value::String(self.attr_name), Value::Type(self.val_type),],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [attr_name, val_type]: [Value; 2usize] = val
                .into_struct()
                .try_into()
                .unwrap();
            Self {
                attr_name: attr_name.into_string(),
                val_type: val_type.into_type(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Validator {
        pub name: String,
        pub attr_name: String,
        pub parent: Option<TypePtr>,
    }
    impl Schema for Validator {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(11);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(11),
                vec![
                    Value::String(self.name), Value::String(self.attr_name),
                    Value::Option(Type::TypePtr, Box::new(self.parent.map(| sv |
                    Value::TypePtr(sv)))),
                ],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [name, attr_name, parent]: [Value; 3usize] = val
                .into_struct()
                .try_into()
                .unwrap();
            Self {
                name: name.into_string(),
                attr_name: attr_name.into_string(),
                parent: parent.into_option().map(|sv| sv.into_type_ptr()),
            }
        }
    }
}
pub mod prim {
    use crate::{types::*, meta::{Timestamp, ObjectPtr, TypePtr}};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UnixTs(pub u64);
    impl Schema for UnixTs {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(1);
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
        const PTR: TypePtr = TypePtr::from_u16_unchecked(4);
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
}
pub mod meta {
    use crate::{types::*, meta::{Timestamp, ObjectPtr, TypePtr}};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Rev {
        Const(Value),
        Mut(Value),
        IterListAdd(Vec<Value>),
        IterSetAdd(Vec<Value>),
        IterSetRemove(Vec<Value>),
    }
    impl Schema for Rev {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(6);
        fn serialize(self) -> Value {
            Value::Enum(
                TypePtr::from_u16_unchecked(6),
                match &self {
                    Self::Const(_) => 0,
                    Self::Mut(_) => 1,
                    Self::IterListAdd(_) => 2,
                    Self::IterSetAdd(_) => 3,
                    Self::IterSetRemove(_) => 4,
                },
                Box::new(
                    match self {
                        Self::Const(val) => val,
                        Self::Mut(val) => val,
                        Self::IterListAdd(val) => {
                            Value::List(
                                Type::Unknown,
                                val.into_iter().map(|sv| sv).collect(),
                            )
                        }
                        Self::IterSetAdd(val) => {
                            Value::List(
                                Type::Unknown,
                                val.into_iter().map(|sv| sv).collect(),
                            )
                        }
                        Self::IterSetRemove(val) => {
                            Value::List(
                                Type::Unknown,
                                val.into_iter().map(|sv| sv).collect(),
                            )
                        }
                    },
                ),
            )
        }
        fn deserialize(val: Value) -> Self {
            let (variant, val) = val.into_enum();
            match variant {
                0 => Self::Const(val),
                1 => Self::Mut(val),
                2 => {
                    Self::IterListAdd(val.into_list().into_iter().map(|sv| sv).collect())
                }
                3 => Self::IterSetAdd(val.into_list().into_iter().map(|sv| sv).collect()),
                4 => {
                    Self::IterSetRemove(
                        val.into_list().into_iter().map(|sv| sv).collect(),
                    )
                }
                _ => unreachable!(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[derive(PartialOrd, Ord)]
    pub struct RevPtr {
        pub object: ObjectPtr,
        pub trait_type: TypePtr,
        pub attr: u8,
    }
    impl Schema for RevPtr {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(7);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(7),
                vec![
                    Value::ObjectPtr(self.object), Value::TypePtr(self.trait_type),
                    Value::UInt8(self.attr),
                ],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [object, trait_type, attr]: [Value; 3usize] = val
                .into_struct()
                .try_into()
                .unwrap();
            Self {
                object: object.into_object_ptr(),
                trait_type: trait_type.into_type_ptr(),
                attr: attr.into_uint8(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[derive(PartialOrd, Ord)]
    pub struct CommitPtr {
        pub ts: Timestamp,
        pub opr: ObjectPtr,
        pub seq: u16,
    }
    impl Schema for CommitPtr {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(8);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(8),
                vec![
                    Value::Timestamp(self.ts), Value::ObjectPtr(self.opr),
                    Value::UInt16(self.seq),
                ],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [ts, opr, seq]: [Value; 3usize] = val.into_struct().try_into().unwrap();
            Self {
                ts: ts.into_timestamp(),
                opr: opr.into_object_ptr(),
                seq: seq.into_uint16(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Commit {
        pub ptr: super::meta::CommitPtr,
        pub revs: Vec<(super::meta::RevPtr, super::meta::Rev)>,
    }
    impl Schema for Commit {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(9);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(9),
                vec![
                    self.ptr.serialize(),
                    Value::Map((Type::Struct(TypePtr::from_u16_unchecked(7)),
                    Type::Struct(TypePtr::from_u16_unchecked(6))), self.revs.into_iter()
                    .map(| (sk, sv) | (sk.serialize(), sv.serialize())).collect()),
                ],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [ptr, revs]: [Value; 2usize] = val.into_struct().try_into().unwrap();
            Self {
                ptr: ptr.deserialize_into(),
                revs: revs
                    .into_map()
                    .into_iter()
                    .map(|(sk, sv)| (sk.deserialize_into(), sv.deserialize_into()))
                    .collect(),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct StateRevPtr {
        pub object: ObjectPtr,
        pub trait_type: TypePtr,
        pub state_attr: u8,
    }
    impl Schema for StateRevPtr {
        const PTR: TypePtr = TypePtr::from_u16_unchecked(12);
        fn serialize(self) -> Value {
            Value::Struct(
                TypePtr::from_u16_unchecked(12),
                vec![
                    Value::ObjectPtr(self.object), Value::TypePtr(self.trait_type),
                    Value::UInt8(self.state_attr),
                ],
            )
        }
        fn deserialize(val: Value) -> Self {
            let [object, trait_type, state_attr]: [Value; 3usize] = val
                .into_struct()
                .try_into()
                .unwrap();
            Self {
                object: object.into_object_ptr(),
                trait_type: trait_type.into_type_ptr(),
                state_attr: state_attr.into_uint8(),
            }
        }
    }
}
