use ::std::collections::BTreeMap;
use once_cell::sync::Lazy;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct Path {
    pub path: &'static str,
    pub name: &'static str,
    // pub ver: &'static str,
}

impl Path {
    pub fn to_path(&self) -> String {
        macros::concat_string!(
            "std",
            ":", self.path,
            ":", self.name
        )
    }

    pub fn to_rust_name(&self) -> String {
        crate::util::to_pascal_case(self.name)
    }

    pub fn to_rust_middle_path(&self) -> String {
        assert!(matches!(self.path.find(':'), None));
        crate::util::to_snake_case(self.path) // .replace(':', "::")
    }

    pub fn to_rust_path(&self) -> String {
        macros::concat_string!(
            self.to_rust_middle_path(),
            "::", self.to_rust_name()
        )
    }

    pub fn to_rust_full_path(&self) -> String {
        macros::concat_string!(
            "zeon::std::codegen",
            "::", self.to_rust_middle_path(),
            "::", self.to_rust_name()
        )
    }
}

// region: deftype macros

macro_rules! def_alias {
    ($ty:expr) => {
        DefType::Alias($ty)
    };
}

macro_rules! def_enum {
    ($($variant:literal)*) => {
        DefType::Enum(vec![
            $(($variant.to_owned(), Unit),)*
        ])
    };
    ($($variant:literal -> $ty:expr)*) => {
        DefType::Enum(vec![
            $(($variant.to_owned(), $ty),)*
        ])
    };
}

macro_rules! def_struct {
    ($($field:literal -> $ty:expr)*) => {
        DefType::Struct(vec![
            $(($field.to_owned(), $ty),)*
        ])
    };
}

macro_rules! deftypes {
    ($($stdptr:literal | std :$path:literal :$name:literal -> $deftype:expr)*) => {
        pub const fn ptr2path(stdptr: u16) -> Option<Path> {
            Some(match stdptr {
                $($stdptr => Path { path: $path, name: $name },)*
                _ => return None,
            })
        }

        pub fn path2ptr(path: Path) -> Option<u16> {
            Some(match path {
                $(Path { path: $path, name: $name } => $stdptr,)*
                _ => return None,
            })
        }

        pub fn init_deftypes() -> BTreeMap<u16, DefType> {
            use crate::types::Type::*;
            macro_rules! list {
                ($ty:expr) => {
                    List(Box::new($ty))
                };
            }
            macro_rules! map {
                ($tyk:expr, $tyv:expr) => {
                    Map(Box::new($tyk), Box::new($tyv))
                };
            }
            macro_rules! ty {
                (:$p:literal :$n:literal) => {
                    TypePtr::from_u16_unchecked(path2ptr(Path { path: $p, name: $n }).unwrap())
                };
            }
            macro_rules! alias_t {
                (:$p:literal :$n:literal) => {
                    Alias(ty!(:$p :$n))
                };
            }
            macro_rules! enum_t {
                (:$p:literal :$n:literal) => {
                    Enum(ty!(:$p :$n))
                };
            }
            macro_rules! struct_t {
                (:$p:literal :$n:literal) => {
                    Struct(ty!(:$p :$n))
                };
            }
            [
                $(($stdptr, $deftype),)*
            ].into_iter().collect()
        }
    };
}

// endregion

// If there is a duplicate ptr, the generated `ptr2path` will raise an `unreachable_patterns` warning
deftypes! {
    0x0000 | std :"types" :"deftype" -> def_enum! {
        "alias"  -> Type
        "enum"   -> map!(String, Type)
        "struct" -> map!(String, Type)
    }
    0x0001 | std :"prim" :"unix-ts" -> def_alias! (UInt)
    0x0002 | std :"types" :"trait-field" -> def_struct! {
        "field-type" -> enum_t!(:"types" :"trait-field-type")
        "name"       -> alias_t!(:"prim" :"simple-name")
        "val-type"   -> Type
    }
    0x0003 | std :"types" :"trait-field-type" -> def_enum! {
        "const"
        "mut"
        "iter"
        "iterset"
        "complex"
    }
    0x0004 | std :"prim" :"simple-name" -> def_alias! (String)
    0x0005 | std :"types" :"trait" -> def_struct! {
        "fields"  -> list!(enum_t!(:"types" :"trait-field"))
        "extends" -> list!(enum_t!(:"meta" :"typeptr"))
    }
    0x0006 | std :"pattern" :"refset-item" -> def_enum! {
        "remove" -> ObjectRef
        "add"    -> ObjectRef
    }
    0x0007 | std :"meta" :"typeptr" -> def_enum! {
        "std"  -> alias_t!(:"meta" :"typeptr-std")
        "hash" -> alias_t!(:"meta" :"typeptr-hash")
    }
    0x0008 | std :"prim" :"u8" -> def_alias! (UInt)
    0x0009 | std :"prim" :"u16" -> def_alias! (UInt)
    0x000A | std :"prim" :"u32" -> def_alias! (UInt)
    0x000B | std :"prim" :"i8" -> def_alias! (Int)
    0x000C | std :"prim" :"i16" -> def_alias! (Int)
    0x000D | std :"prim" :"i32" -> def_alias! (Int)
    0x000E | std :"meta" :"typeptr-std" -> def_alias! (UInt)
    0x000F | std :"meta" :"typeptr-hash" -> def_alias! (UInt)
    0x0010 | std :"meta" :"object-type" -> def_alias! (UInt)
    0x0011 | std :"meta" :"object-id" -> def_alias! (UInt)
}

/*
deftraits! {
    0x8000 | std :"meta" :"name" -> def_trait! {
        Mut "name" -> alias_t!(:"prim" :"simple-name")
    } // extends ty!(...) + ty!(..)
}
*/

pub static DEFTYPES: Lazy<BTreeMap<u16, DefType>> = Lazy::new(init_deftypes);

pub mod codegen;
pub mod casting;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(ptr2path(0x0001).unwrap().to_path(), "std:prim:unix-ts");
        assert_eq!(ptr2path(0x0001).unwrap().to_rust_name(), "UnixTs");
        assert_eq!(ptr2path(0x0001).unwrap().to_rust_path(), "prim::UnixTs");
        assert_eq!(ptr2path(0x0001).unwrap().to_rust_full_path(), "zeon::std::codegen::prim::UnixTs");
        assert_eq!(path2ptr(Path { path: "prim", name: "unix-ts" }).unwrap(), 0x0001);
        assert_eq!(DEFTYPES.get(&0x0001).unwrap().clone(), DefType::Alias(Type::UInt));
    }
}
