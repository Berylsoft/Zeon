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
            self.path,
            ":", self.name
        )
    }

    // pub fn to_rustpath(&self) -> String {
    //     macros::concat_string!(
    //         "zeon::std::codegen",
    //         crate::util::to_rust_path(self.path).replace(":", "::"),
    //         "::", crate::util::to_rust_name(self.name)
    //     )
    // }

    // pub fn to_rustname(&self) -> String {
    //     crate::util::to_rust_name(self.name)
    // }

    pub fn to_rustpath(&self) -> String {
        macros::concat_string!(
            "zeon::std::codegen",
            "::", crate::util::to_snake_case(self.path).replace(":", "_").replacen("_", "", 1),
            "_", crate::util::to_pascal_case(self.name)
        )
    }

    pub fn to_rustname(&self) -> String {
        macros::concat_string!(
            crate::util::to_snake_case(self.path).replace(":", "_").replacen("_", "", 1),
            "_", crate::util::to_pascal_case(self.name)
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
    ($($stdptr:literal | std $path:literal :$name:literal -> $deftype:expr)*) => {
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
            macro_rules! ptr {
                ($n:expr) => {
                    TypePtr::from_u16_unchecked($n)
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
    0x0000 | std ":types" :"deftype" -> def_enum! {
        "alias"  -> Type
        "enum"   -> map!(String, Type)
        "struct" -> map!(String, Type)
    }
    0x0001 | std ":prim" :"unix-ts" -> def_alias! (UInt)
    0x0002 | std ":types" :"trait-field" -> def_struct! {
        "field-type" -> Enum(ptr!(0x0003))
        "name"       -> Alias(ptr!(0x0004))
        "val-type"   -> Type
    }
    0x0003 | std ":types" :"trait-field-type" -> def_enum! {
        "const"
        "mut"
        "iter"
    }
    0x0004 | std ":prim" :"simple-name" -> def_alias! (String)
    0x0005 | std ":types" :"trait" -> def_struct! {
        "fields" -> list!(Enum(ptr!(0x0002)))
    }
    0x0006 | std ":pattern" :"refset-item" -> def_enum! {
        "remove" -> ObjectRef
        "add"    -> ObjectRef
    }
}

pub static DEFTYPES: Lazy<BTreeMap<u16, DefType>> = Lazy::new(init_deftypes);

pub mod codegen;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(ptr2path(0x0001).unwrap().to_path(), "std:prim:unix-ts");
        // assert_eq!(ptr2path(0x0001).unwrap().to_rustpath(), "zeon::std::codegen::prim::UnixTs");
        // assert_eq!(ptr2path(0x0001).unwrap().to_rustname(), "UnixTs");
        assert_eq!(ptr2path(0x0001).unwrap().to_rustpath(), "zeon::std::codegen::prim_UnixTs");
        assert_eq!(ptr2path(0x0001).unwrap().to_rustname(), "prim_UnixTs");
        assert_eq!(path2ptr(Path { path: ":prim", name: "unix-ts" }).unwrap(), 0x0001);
        assert_eq!(DEFTYPES.get(&0x0001).unwrap().clone(), DefType::Alias(Type::UInt));
    }
}
