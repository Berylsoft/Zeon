#![allow(unused_macros)]

use std::collections::BTreeMap;
use crate::{types::{self, *}, meta};

pub mod path;
use path::StdPath;

pub struct Std {
    pub types: BTreeMap<u16, DefType>,
    pub traits: BTreeMap<u16, Trait>,
}

// region: macros

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

macro_rules! ref_type {
    (:$p:literal :$n:literal) => {{
        const P: meta::TypePtr = {
            let p = const_path2ptr(StdPath { path: $p, name: $n });
            assert!(p < 0x8000);
            meta::TypePtr::from_u16_unchecked(p)
        };
        P
    }};
}

macro_rules! ref_trait {
    (:$p:literal :$n:literal) => {{
        const P: meta::TypePtr = {
            let p = const_path2ptr(StdPath { path: $p, name: $n });
            assert!(p >= 0x8000);
            meta::TypePtr::from_u16_unchecked(p)
        };
        P
    }};
}

macro_rules! ref_alias {
    (:$p:literal :$n:literal) => {
        Alias(ref_type!(:$p :$n))
    };
}

macro_rules! ref_c_enum {
    (:$p:literal :$n:literal) => {
        CEnum(ref_type!(:$p :$n))
    };
}

macro_rules! ref_enum {
    (:$p:literal :$n:literal) => {
        Enum(ref_type!(:$p :$n))
    };
}

macro_rules! ref_struct {
    (:$p:literal :$n:literal) => {
        Struct(ref_type!(:$p :$n))
    };
}

macro_rules! def_alias {
    ($ty:expr) => {
        DefType::Alias($ty)
    };
}

macro_rules! def_c_enum {
    ($($variant:literal)*) => {
        DefType::CEnum(vec![
            $($variant.to_owned(),)*
        ])
    };
}

macro_rules! def_enum {
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

macro_rules! def_trait {
    ($($attr_type:ident $attr_name:literal -> $val_type:expr)* $(;extends $($extend:expr,)*)*) => {
        Trait {
            attrs: vec![$(TraitAttr {
                attr_type: TraitAttrType::$attr_type,
                attr_name: $attr_name.to_owned(),
                val_type: $val_type,
            })*],
            extends: vec![$($($extend,)*)*]
        }
    };
}

macro_rules! def_std {
    {
        types { $($stdptr:literal | std :$path:literal :$name:literal -> $deftype:expr)* }
        traits { $($stdptr2:literal | std :$path2:literal :$name2:literal -> $deftrait:expr)* }
    } => {
        #[deny(unreachable_patterns)] // deny duplicate ptr
        pub const fn ptr2path(stdptr: u16) -> Option<StdPath> {
            Some(match stdptr {
                $($stdptr => StdPath { path: $path, name: $name },)*
                $($stdptr2 => StdPath { path: $path2, name: $name2 },)*
                _ => return None,
            })
        }

        pub fn path2ptr(path: StdPath) -> Option<u16> {
            Some(match path {
                $(StdPath { path: $path, name: $name } => $stdptr,)*
                $(StdPath { path: $path2, name: $name2 } => $stdptr2,)*
                _ => return None,
            })
        }

        const fn const_path2ptr(path: StdPath) -> u16 {
            $(if (
                crate::util::const_str_equal(path.path, $path) &
                crate::util::const_str_equal(path.name, $name)
            ) {
                return $stdptr
            })*
            $(if (
                crate::util::const_str_equal(path.path, $path2) &
                crate::util::const_str_equal(path.name, $name2)
            ) {
                return $stdptr2
            })*
            unreachable!();
        }

        pub fn init() -> Std {
            use types::Type::*;
            Std {
                types: [$(($stdptr, $deftype),)*].into_iter().collect(),
                traits: [$(($stdptr2, $deftrait),)*].into_iter().collect(),
            }
        }
    };
}

// endregion

def_std! {
    types {
        0x0000 | std :"types" :"deftype" -> def_enum! {
            "alias"  -> Type
            "c-enum" -> list!(String /* simple-name */)
            "enum"   -> map!(String /* simple-name */, Type)
            "struct" -> map!(String /* simple-name */, Type)
        }
        0x0001 | std :"prim" :"unix-ts" -> def_alias! (UInt)
        0x0002 | std :"types" :"trait-attr" -> def_struct! {
            "attr-type" -> ref_c_enum!(:"types" :"trait-attr-type")
            "attr-name" -> String /* simple-name */
            "val-type"  -> Type
        }
        0x0003 | std :"types" :"trait-attr-type" -> def_c_enum! {
            "const"
            "mut"
            "iter-list"
            "iter-set"
            "complex"
        }
        0x0004 | std :"prim" :"simple-name" -> def_alias! (String)
        0x0005 | std :"types" :"trait" -> def_struct! {
            "attrs"   -> list!(ref_enum!(:"types" :"trait-attr"))
            "extends" -> list!(TypePtr)
        }
        0x0006 | std :"meta" :"rev-type" -> def_c_enum! {
            "const"
            "mut"
            "iter-list-add"
            "iter-set-add"
            "iter-set-remove"
            "complex"
        }
        0x0007 | std :"meta" :"rev-ptr" -> def_struct! {
            "object"     -> ObjectPtr
            "trait-type" -> TypePtr
            "attr"       -> UInt8
            "rev-type"   -> ref_c_enum!(:"meta" :"rev-type")
        }
        0x0008 | std :"meta" :"commit-ptr" -> def_struct! {
            "ts"  -> Timestamp
            "opr" -> ObjectPtr /* impl std:opr:operator */
            "seq" -> UInt16 // reserved for cluster randgen
        }
        0x0009 | std :"meta" :"commit" -> def_struct! {
            "ptr" -> ref_struct!(:"meta" :"commit-ptr")
            "hash" -> Bytes
            "revs" -> map!(ref_struct!(:"meta" :"rev-ptr"), Unknown) // map unique?
        }
    }
    traits {
        0x8000 | std :"meta" :"object-meta" -> def_trait! {
            IterSet "traits" -> TypePtr
        }
        0x8001 | std :"meta" :"name" -> def_trait! {
            Mut "name" -> ref_alias!(:"prim" :"simple-name")
        }
        0x8002 | std :"meta" :"unique-name" -> def_trait! {
            ;extends ref_trait!(:"meta" :"name"),
        }
    }
}

pub mod codegen;
pub mod casting;
pub mod check;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let std = init();
        assert_eq!(ptr2path(0x0001).unwrap().to_path(), "std:prim:unix-ts");
        assert_eq!(ptr2path(0x0001).unwrap().to_rust_name(), "UnixTs");
        assert_eq!(ptr2path(0x0001).unwrap().to_rust_self_path(), "super::prim::UnixTs");
        assert_eq!(ptr2path(0x0001).unwrap().to_rust_foreign_path(), "zeon::std::codegen::prim::UnixTs");
        assert_eq!(path2ptr(StdPath { path: "prim", name: "unix-ts" }).unwrap(), 0x0001);
        assert_eq!(const_path2ptr(StdPath { path: "prim", name: "unix-ts" }), 0x0001);
        assert_eq!(std.types.get(&0x0001).unwrap().clone(), DefType::Alias(Type::UInt));
        assert_eq!(format!("{:?}", std.traits.get(&0x8000).unwrap().clone()), r#"Trait { attrs: [TraitAttr { attr_type: IterSet, attr_name: "traits", val_type: TypePtr }], extends: [] }"#);
        assert_eq!(format!("{:?}", std.traits.get(&0x8001).unwrap().clone()), r#"Trait { attrs: [TraitAttr { attr_type: Mut, attr_name: "name", val_type: Alias(Std(StdPtr(4))) }], extends: [] }"#);
    }
}
