use super::*;

impl Type {
    pub fn as_tag(&self) -> Tag {
        match self {
            Type::Unit      => Tag::Unit,
            Type::Bool      => Tag::Bool,
            Type::Int       => Tag::Int,
            Type::UInt      => Tag::UInt,
            Type::Float     => Tag::Float,
            Type::String    => Tag::String,
            Type::Bytes     => Tag::Bytes,
            Type::Option(_) => Tag::Option,
            Type::List(_)   => Tag::List,
            Type::Set(_)    => Tag::Set,
            Type::Map(_, _) => Tag::Map,
            Type::Alias(_)  => Tag::Alias,
            Type::Enum(_)   => Tag::Enum,
            Type::Tuple(_)  => Tag::Tuple,
            Type::Struct(_) => Tag::Struct,
            Type::Type      => Tag::Type,
        }
    }
}

impl Value {
    pub fn as_tag(&self) -> Tag {
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
            Value::Set(_, _)     => Tag::Set,
            Value::Map(_, _)     => Tag::Map,
            Value::Alias(_, _)   => Tag::Alias,
            Value::Enum(_, _, _) => Tag::Enum,
            Value::Tuple(_, _)   => Tag::Tuple,
            Value::Struct(_, _)  => Tag::Struct,
            Value::Type(_)       => Tag::Type,
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
            Value::Set(t, _) => Type::Set(Box::new(t.clone())),
            Value::Map((tk, tv), _) => Type::Map(Box::new(tk.clone()), Box::new(tv.clone())),
            Value::Alias(ptr, _) => Type::Alias(*ptr),
            Value::Enum(ptr, _, _) => Type::Enum(*ptr),
            Value::Tuple(ptr, _) => Type::Tuple(*ptr),
            Value::Struct(ptr, _) => Type::Struct(*ptr),
            Value::Type(_) => Type::Type,
        }
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
            return v;
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

    pub fn into_set(self) -> Vec<Value> {
        if let Value::Set(_t, s) = self {
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

    pub fn into_alias(self) -> Value {
        if let Value::Alias(_ptr, v) = self {
            return *v;
        }
        unreachable!()
    }

    pub fn into_enum(self) -> (EnumVarient, Value) {
        if let Value::Enum(_ptr, ev, v) = self {
            return (ev, *v);
        }
        unreachable!()
    }

    pub fn into_tuple(self) -> Vec<Value> {
        if let Value::Tuple(_ptr, s) = self {
            return s;
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
}

impl Type {
    pub fn into_option(self) -> Type {
        if let Type::Option(v) = self {
            return *v;
        }
        unreachable!()
    }

    pub fn into_list(self) -> Type {
        if let Type::List(v) = self {
            return *v;
        }
        unreachable!()
    }

    pub fn into_set(self) -> Type {
        if let Type::Set(v) = self {
            return *v;
        }
        unreachable!()
    }

    pub fn into_map(self) -> (Type, Type) {
        if let Type::Map(k, v) = self {
            return (*k, *v);
        }
        unreachable!()
    }

    pub fn into_alias(self) -> TypePtr {
        if let Type::Alias(ptr) = self {
            return ptr;
        }
        unreachable!()
    }

    pub fn into_enum(self) -> TypePtr {
        if let Type::Enum(ptr) = self {
            return ptr;
        }
        unreachable!()
    }

    pub fn into_tuple(self) -> TypePtr {
        if let Type::Tuple(ptr) = self {
            return ptr;
        }
        unreachable!()
    }

    pub fn into_struct(self) -> TypePtr {
        if let Type::Struct(ptr) = self {
            return ptr;
        }
        unreachable!()
    }
}

impl DefType {
    pub fn alias_inner(&self) -> Type {
        if let DefType::Alias(t) = self {
            return t.clone();
        }
        unreachable!()
    }

    pub fn enum_inner(&self, ev: EnumVarient) -> Type {
        if let DefType::Enum(s) = self {
            return s[ev as usize].1.clone();
        }
        unreachable!()
    }

    pub fn tuple_inner(&self, pos: usize) -> Type {
        if let DefType::Tuple(s) = self {
            return s[pos].clone();
        }
        unreachable!()
    }

    pub fn struct_inner(&self, pos: usize) -> Type {
        if let DefType::Struct(s) = self {
            return s[pos].1.clone();
        }
        unreachable!()
    }
}
