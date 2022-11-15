use super::*;

impl Type {
    pub const fn as_tag(&self) -> Tag {
        match self {
            Type::Unknown   => Tag::Unknown,
            Type::Unit      => Tag::Unit,
            Type::Bool      => Tag::Bool,
            Type::Int       => Tag::Int,
            Type::UInt      => Tag::UInt,
            Type::Float     => Tag::Float,
            Type::String    => Tag::String,
            Type::Bytes     => Tag::Bytes,
            Type::Option(_) => Tag::Option,
            Type::List(_)   => Tag::List,
            Type::Map(_, _) => Tag::Map,
            Type::Tuple(_)  => Tag::Tuple,
            Type::Alias(_)  => Tag::Alias,
            Type::Enum(_)   => Tag::Enum,
            Type::CEnum(_)  => Tag::CEnum,
            Type::Struct(_) => Tag::Struct,
            Type::Type      => Tag::Type,
            Type::TypePtr   => Tag::TypePtr,
            Type::ObjectPtr => Tag::ObjectPtr,
            Type::Timestamp => Tag::Timestamp,
        }
    }
}

impl Value {
    pub const fn as_tag(&self) -> Tag {
        match self {
            Value::Unit          => Tag::Unit,
            Value::Bool(_)       => Tag::Bool,
            Value::Int(_)        => Tag::Int,
            Value::UInt(_)       => Tag::UInt,
            Value::Float(_)      => Tag::Float,
            Value::String(_)     => Tag::String,
            Value::Bytes(_)      => Tag::Bytes,
            Value::Option(_, _)  => Tag::Option,
            Value::List(_, _)    => Tag::List,
            Value::Map(_, _)     => Tag::Map,
            Value::Tuple(_)      => Tag::Tuple,
            Value::Alias(_, _)   => Tag::Alias,
            Value::CEnum(_, _)   => Tag::CEnum,
            Value::Enum(_, _, _) => Tag::Enum,
            Value::Struct(_, _)  => Tag::Struct,
            Value::Type(_)       => Tag::Type,
            Value::TypePtr(_)    => Tag::TypePtr,
            Value::ObjectPtr(_)  => Tag::ObjectPtr,
            Value::Timestamp(_)  => Tag::Timestamp,
        }
    }

    pub const fn as_htag(&self) -> HTag {
        match self {
            Value::Int(_)        => HTag::Int,
            Value::UInt(_)       => HTag::UInt,
            Value::Float(_)      => HTag::Float,
            Value::String(_)     => HTag::String,
            Value::Bytes(_)      => HTag::Bytes,
            Value::List(_, _)    => HTag::List,
            Value::Map(_, _)     => HTag::Map,
            Value::Tuple(_)      => HTag::Tuple,
            Value::CEnum(_, _)   => HTag::CEnum,
            Value::Enum(_, _, _) => HTag::Enum,
            Value::Struct(_, _)  => HTag::Struct,

            Value::Unit          |
            Value::Bool(_)       |
            Value::Option(_, _)  |
            Value::Alias(_, _)   |
            Value::Type(_)       |
            Value::TypePtr(_)    |
            Value::ObjectPtr(_)  |
            Value::Timestamp(_)  => HTag::L4,
        }
    }

    pub fn as_type(&self) -> Type {
        match self {
            Value::Unit => Type::Unit,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::UInt(_) => Type::UInt,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Bytes(_) => Type::Bytes,
            Value::Option(t, _) => Type::Option(Box::new(t.clone())),
            Value::List(t, _) => Type::List(Box::new(t.clone())),
            Value::Map((tk, tv), _) => Type::Map(Box::new(tk.clone()), Box::new(tv.clone())),
            Value::Tuple(seq) => Type::Tuple(seq.iter().map(|v| v.as_type()).collect()),
            Value::Alias(ptr, _) => Type::Alias(*ptr),
            Value::CEnum(ptr, _) => Type::CEnum(*ptr),
            Value::Enum(ptr, _, _) => Type::Enum(*ptr),
            Value::Struct(ptr, _) => Type::Struct(*ptr),
            Value::Type(_) => Type::Type,
            Value::TypePtr(_) => Type::TypePtr,
            Value::ObjectPtr(_) => Type::ObjectPtr,
            Value::Timestamp(_) => Type::Timestamp,
        }
    }
}

impl Value {
    pub fn serialize_from<T: Schema>(val: T) -> Value {
        val.serialize()
    }

    pub fn deserialize_into<T: Schema>(self) -> T {
        T::deserialize(self)
    }
}

impl Value {
    pub fn from_float(v: f64) -> Value {
        Value::Float(v.to_bits())
    }
}

impl Value {
    pub fn into_unit(self) {
        if let Value::Unit = self {
            return;
        }
        unreachable!()
    }

    pub fn into_bool(self) -> bool {
        if let Value::Bool(v) = self {
            return v;
        }
        unreachable!()
    }

    pub fn into_int(self) -> i64 {
        if let Value::Int(v) = self {
            return v;
        }
        unreachable!()
    }

    pub fn into_uint(self) -> u64 {
        if let Value::UInt(v) = self {
            return v;
        }
        unreachable!()
    }

    pub fn into_float(self) -> f64 {
        if let Value::Float(v) = self {
            return f64::from_bits(v);
        }
        unreachable!()
    }

    pub fn into_string(self) -> String {
        if let Value::String(v) = self {
            return v;
        }
        unreachable!()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        if let Value::Bytes(v) = self {
            return v;
        }
        unreachable!()
    }

    pub fn into_option(self) -> Option<Value> {
        if let Value::Option(_t, v) = self {
            return *v;
        }
        unreachable!()
    }

    pub fn into_list(self) -> Vec<Value> {
        if let Value::List(_t, s) = self {
            return s;
        }
        unreachable!()
    }

    pub fn into_map(self) -> Vec<(Value, Value)> {
        if let Value::Map(_t, s) = self {
            return s;
        }
        unreachable!()
    }

    pub fn into_tuple(self) -> Vec<Value> {
        if let Value::Tuple(s) = self {
            return s;
        }
        unreachable!()
    }

    pub fn into_alias(self) -> Value {
        if let Value::Alias(_ptr, v) = self {
            return *v;
        }
        unreachable!()
    }

    pub fn into_c_enum(self) -> EnumVariant {
        if let Value::CEnum(_ptr, ev) = self {
            return ev;
        }
        unreachable!()
    }

    pub fn into_enum(self) -> (EnumVariant, Value) {
        if let Value::Enum(_ptr, ev, v) = self {
            return (ev, *v);
        }
        unreachable!()
    }

    pub fn into_struct(self) -> Vec<Value> {
        if let Value::Struct(_ptr, s) = self {
            return s;
        }
        unreachable!()
    }

    pub fn into_type(self) -> Type {
        if let Value::Type(v) = self {
            return v;
        }
        unreachable!()
    }

    pub fn into_type_ptr(self) -> TypePtr {
        if let Value::TypePtr(v) = self {
            return v;
        }
        unreachable!()
    }

    pub fn into_object_ptr(self) -> ObjectPtr {
        if let Value::ObjectPtr(v) = self {
            return v;
        }
        unreachable!()
    }

    pub fn into_timestamp(self) -> Timestamp {
        if let Value::Timestamp(v) = self {
            return v;
        }
        unreachable!()
    }
}
