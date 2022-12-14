use proc_macro2::{TokenStream, Literal};
use quote::quote;
use foundations::{concat_string, case_convert::*};
use zeon::{types::{Type, DefType, EnumVariantId}, meta::TypePtr, std::{ptr2path, init, Std}};

fn ident<S: AsRef<str>>(s: S) -> TokenStream {
    s.as_ref().parse().unwrap()
}

fn ptr2rustname(ptr: TypePtr) -> TokenStream {
    let ptr = ptr.as_std_inner().unwrap();
    let path = ptr2path(ptr).unwrap();
    ident(path.to_rust_name())
}

fn ptr2rustpath(ptr: TypePtr) -> TokenStream {
    let ptr = ptr.as_std_inner().unwrap();
    let path = ptr2path(ptr).unwrap();
    ident(path.to_rust_self_path())
}

fn ptr2tokens(ptr: TypePtr) -> TokenStream {
    match ptr {
        TypePtr::Std(ptr) => {
            let ptr_literal = Literal::u16_unsuffixed(ptr.to_u16());
            quote!(TypePtr::from_u16_unchecked(#ptr_literal))
        },
        TypePtr::Hash(hash) => {
            let hex = hex::encode(hash);
            quote!(TypePtr::Hash(hex_literal::hex!(#hex)))
        },
    }
}

fn type2tokens(ty: Type) -> TokenStream {
    match ty {
        Type::Unknown => quote!(Type::Unknown),
        Type::Unit => quote!(Type::Unit),
        Type::Bool => quote!(Type::Bool),
        Type::Int => quote!(Type::Int),
        Type::UInt => quote!(Type::UInt),
        Type::Float => quote!(Type::Float),
        Type::String => quote!(Type::String),
        Type::Bytes => quote!(Type::Bytes),
        Type::Type => quote!(Type::Type),
        Type::TypePtr => quote!(Type::TypePtr),
        Type::ObjectPtr => quote!(Type::ObjectPtr),
        Type::Timestamp => quote!(Type::Timestamp),
        Type::UInt8 => quote!(Type::UInt8),
        Type::UInt16 => quote!(Type::UInt16),
        Type::UInt32 => quote!(Type::UInt32),

        Type::Option(sty) => {
            let sty = type2tokens(*sty);
            quote!(Type::Option(Box::new(#sty)))
        },
        Type::List(sty) => {
            let sty = type2tokens(*sty);
            quote!(Type::List(Box::new(#sty)))
        },
        Type::Map(styk, styv) => {
            let styk = type2tokens(*styk);
            let styv = type2tokens(*styv);
            quote!(Type::Map(Box::new(#styk), Box::new(#styv)))
        },
        Type::Tuple(stys) => {
            let stys = stys.into_iter().map(type2tokens);
            quote!(Type::Tuple(vec![#(#stys,)*]))
        },

        Type::Alias(ptr) => {
            let ptr = ptr2tokens(ptr);
            quote!(Type::Alias(#ptr))
        },
        Type::CEnum(ptr) => {
            let ptr = ptr2tokens(ptr);
            quote!(Type::CEnum(#ptr))
        },
        Type::Enum(ptr) => {
            let ptr = ptr2tokens(ptr);
            quote!(Type::Enum(#ptr))
        },
        Type::Struct(ptr) => {
            let ptr = ptr2tokens(ptr);
            quote!(Type::Struct(#ptr))
        },
    }
}

fn type2type(ty: Type) -> TokenStream {
    match ty {
        Type::Unknown => quote!(Value),
        Type::Unit => quote!(()),
        Type::Bool => quote!(bool),
        Type::Int => quote!(i64),
        Type::UInt => quote!(u64),
        Type::Float => quote!(f64),
        Type::String => quote!(String),
        Type::Bytes => quote!(Vec<u8>),
        Type::Type => quote!(Type),
        Type::TypePtr => quote!(TypePtr),
        Type::ObjectPtr => quote!(ObjectPtr),
        Type::Timestamp => quote!(Timestamp),
        Type::UInt8 => quote!(u8),
        Type::UInt16 => quote!(u16),
        Type::UInt32 => quote!(u32),

        Type::Option(sty) => {
            let sty = type2type(*sty);
            quote!(Option<#sty>)
        },
        Type::List(sty) => {
            let sty = type2type(*sty);
            quote!(Vec<#sty>)
        },
        Type::Map(styk, styv) => {
            let styk = type2type(*styk);
            let styv = type2type(*styv);
            quote!(Vec<(#styk, #styv)>)
        },
        Type::Tuple(stys) => {
            let stys = stys.into_iter().map(type2type);
            quote!((#(#stys,)*))
        },
        
        Type::Alias(ptr) |
        Type::CEnum(ptr) |
        Type::Enum(ptr) |
        Type::Struct(ptr) => ptr2rustpath(ptr),
    }
}

fn type2de(ty: Type, v: TokenStream) -> TokenStream {
    match ty {
        Type::Unknown => quote!(#v),
        Type::Unit => quote!(#v.into_unit()),
        Type::Bool => quote!(#v.into_bool()),
        Type::Int => quote!(#v.into_int()),
        Type::UInt => quote!(#v.into_uint()),
        Type::Float => quote!(#v.into_float()),
        Type::String => quote!(#v.into_string()),
        Type::Bytes => quote!(#v.into_bytes()),
        Type::Type => quote!(#v.into_type()),
        Type::TypePtr => quote!(#v.into_type_ptr()),
        Type::ObjectPtr => quote!(#v.into_object_ptr()),
        Type::Timestamp => quote!(#v.into_timestamp()),
        Type::UInt8 => quote!(#v.into_uint8()),
        Type::UInt16 => quote!(#v.into_uint16()),
        Type::UInt32 => quote!(#v.into_uint32()),

        Type::Option(sty) => {
            let sty = type2de(*sty, quote!(sv));
            quote!(#v.into_option().map(|sv| #sty))
        },
        Type::List(sty) => {
            let sty = type2de(*sty, quote!(sv));
            quote!(#v.into_list().into_iter().map(|sv| #sty).collect())
        },
        Type::Map(styk, styv) => {
            let styk = type2de(*styk, quote!(sk));
            let styv = type2de(*styv, quote!(sv));
            quote!(#v.into_map().into_iter().map(|(sk, sv)| (#styk, #styv)).collect())
        },
        Type::Tuple(stys) => {
            let stys = stys.into_iter().enumerate().map(|(i, sty)| type2de(sty, quote!(sv[#i])));
            quote!({
                let sv = #v.into_tuple();
                (#(#stys,)*)
            })
        }

        Type::Alias(_) |
        Type::CEnum(_) |
        Type::Enum(_) |
        Type::Struct(_) => quote!(#v.deserialize_into()),
    }
}

fn type2ser(ty: Type, v: TokenStream) -> TokenStream {
    match ty {
        Type::Unknown => quote!(#v),
        Type::Unit => quote!({ let _ = #v; Value::Unit }),
        Type::Bool => quote!(Value::Bool(#v)),
        Type::Int => quote!(Value::Int(#v)),
        Type::UInt => quote!(Value::UInt(#v)),
        Type::Float => quote!(Value::from_float(#v)),
        Type::String => quote!(Value::String(#v)),
        Type::Bytes => quote!(Value::Bytes(#v)),
        Type::Type => quote!(Value::Type(#v)),
        Type::TypePtr => quote!(Value::TypePtr(#v)),
        Type::ObjectPtr => quote!(Value::ObjectPtr(#v)),
        Type::Timestamp => quote!(Value::Timestamp(#v)),
        Type::UInt8 => quote!(Value::UInt8(#v)),
        Type::UInt16 => quote!(Value::UInt16(#v)),
        Type::UInt32 => quote!(Value::UInt32(#v)),

        Type::Option(sty) => {
            let sty_ty = type2tokens(*sty.clone());
            let sty = type2ser(*sty, quote!(sv));
            quote!(Value::Option(#sty_ty, Box::new(#v.map(|sv| #sty))))
        },
        Type::List(sty) => {
            let sty_ty = type2tokens(*sty.clone());
            let sty = type2ser(*sty, quote!(sv));
            quote!(Value::List(#sty_ty, #v.into_iter().map(|sv| #sty).collect()))
        },
        Type::Map(styk, styv) => {
            let styk_ty = type2tokens(*styk.clone());
            let styv_ty = type2tokens(*styv.clone());
            let styk = type2ser(*styk, quote!(sk));
            let styv = type2ser(*styv, quote!(sv));
            quote!(Value::Map((#styk_ty, #styv_ty), #v.into_iter().map(|(sk, sv)| (#styk, #styv)).collect()))
        },
        Type::Tuple(stys) => {
            let stys = stys.into_iter().enumerate().map(|(i, sty)| type2ser(sty, quote!(sv.#i)));
            quote!(Value::Tuple(vec![#(#stys,)*]))
        },

        Type::Alias(_) |
        Type::CEnum(_) |
        Type::Enum(_) |
        Type::Struct(_) => quote!(#v.serialize()),
    }
}

fn derive_def(ptr: u16, dt: DefType) -> TokenStream {
    let ptr_literal = Literal::u16_unsuffixed(ptr);
    match dt {
        DefType::Alias(ty) => {
            let name = ptr2rustname(TypePtr::from_u16_unchecked(ptr));
            let ser = type2ser(ty.clone(), quote!(self.0));
            let de = type2de(ty.clone(), quote!(val));
            let ty = type2type(ty);
            quote!(
                #[derive(Clone, Debug, PartialEq, Eq)]
                pub struct #name(pub #ty);

                impl Schema for #name {
                    const PTR: TypePtr = TypePtr::from_u16_unchecked(#ptr_literal);

                    fn serialize(self) -> Value {
                        #ser
                    }

                    fn deserialize(val: Value) -> Self {
                        Self(#de)
                    }
                }

                impl From<#ty> for #name {
                    fn from(val: #ty) -> Self {
                        Self(val)
                    }
                }

                impl From<#name> for #ty {
                    fn from(val: #name) -> #ty {
                        val.0
                    }
                }
            )
        },
        DefType::CEnum(names) => {
            let len: EnumVariantId = names.len().try_into().unwrap();
            let name = ptr2rustname(TypePtr::from_u16_unchecked(ptr));
            let names = names.into_iter().map(|name| ident(to_pascal_case(&name)));
            let names2 = names.clone();
            let names4 = names.clone();
            let i = (0..len).map(Literal::u64_unsuffixed);
            let i2 = i.clone();

            quote!(
                #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                pub enum #name {
                    #(#names,)*
                }

                impl Schema for #name {
                    const PTR: TypePtr = TypePtr::from_u16_unchecked(#ptr_literal);

                    fn serialize(self) -> Value {
                        Value::CEnum(
                            TypePtr::from_u16_unchecked(#ptr_literal),
                            match &self {
                                #(Self::#names2 => #i,)*
                            },
                        )
                    }

                    fn deserialize(val: Value) -> Self {
                        let variant = val.into_c_enum();
                        match variant {
                            #(#i2 => Self::#names4,)*
                            _ => unreachable!(),
                        }
                    }
                }
            )
        },
        DefType::Enum(variants) => {
            let len: EnumVariantId = variants.len().try_into().unwrap();
            let name = ptr2rustname(TypePtr::from_u16_unchecked(ptr));
            let (names, tys): (Vec<String>, Vec<Type>) = variants.into_iter().unzip();
            let names = names.into_iter().map(|name| ident(to_pascal_case(&name)));
            let names2 = names.clone();
            let names3 = names.clone();
            let names4 = names.clone();
            let i = (0..len).map(Literal::u64_unsuffixed);
            let i2 = i.clone();
            let sers = tys.clone().into_iter().map(|ty| type2ser(ty, quote!(val)));
            let des = tys.clone().into_iter().map(|ty| type2de(ty, quote!(val)));
            let tys = tys.into_iter().map(type2type);

            quote!(
                #[derive(Clone, Debug, PartialEq, Eq)]
                pub enum #name {
                    #(#names(#tys),)*
                }

                impl Schema for #name {
                    const PTR: TypePtr = TypePtr::from_u16_unchecked(#ptr_literal);

                    fn serialize(self) -> Value {
                        Value::Enum(
                            TypePtr::from_u16_unchecked(#ptr_literal),
                            match &self {
                                #(Self::#names2(_) => #i,)*
                            },
                            Box::new(match self {
                                #(Self::#names3(val) => #sers,)*
                            }),
                        )
                    }

                    fn deserialize(val: Value) -> Self {
                        let (variant, val) = val.into_enum();
                        match variant {
                            #(#i2 => Self::#names4(#des),)*
                            _ => unreachable!(),
                        }
                    }
                }
            )
        },
        DefType::Struct(fields) => {
            let name = ptr2rustname(TypePtr::from_u16_unchecked(ptr));
            let len = fields.len();
            let (names, tys): (Vec<String>, Vec<Type>) = fields.clone().into_iter().unzip();
            let names = names.into_iter().map(|name| ident(to_snake_case(&name)));
            let names2 = names.clone();
            let names3 = names.clone();
            let sers = fields.clone().into_iter().map(|(name, ty)| type2ser(ty, ident(concat_string!("self.", to_snake_case(&name)))));
            let des = fields.into_iter().map(|(name, ty)| type2de(ty, ident(to_snake_case(&name))));
            let tys = tys.into_iter().map(type2type);

            let ord = if [
                0x0007, // std:meta:rev-ptr
                0x0008, // std:meta:commit-ptr
                0x000D, // std:meta:state-rev-ptr
            ].contains(&ptr) {
                quote!(#[derive(PartialOrd, Ord)])
            } else {
                quote!()
            };

            quote!(
                #[derive(Clone, Debug, PartialEq, Eq)]
                #ord
                pub struct #name {
                    #(pub #names: #tys,)*
                }

                impl Schema for #name {
                    const PTR: TypePtr = TypePtr::from_u16_unchecked(#ptr_literal);

                    fn serialize(self) -> Value {
                        Value::Struct(TypePtr::from_u16_unchecked(#ptr_literal), vec![
                            #(#sers,)*
                        ])
                    }

                    fn deserialize(val: Value) -> Self {
                        let [#(#names2,)*]: [Value; #len] = val.into_struct().try_into().unwrap();
                        Self {
                            #(#names3: #des,)*
                        }
                    }
                }
            )
        },
    }
}

fn derive_file() -> TokenStream {
    use indexmap::{IndexMap, map::Entry};
    let Std { types, .. } = init();
    let mut map = IndexMap::new();
    for ptr in types.keys() {
        let path = ptr2path(*ptr).unwrap().to_rust_path();
        if let Entry::Vacant(e) = map.entry(path) {
            e.insert(Vec::new());
        }
    }
    for (ptr, dt) in types {
        let path = ptr2path(ptr).unwrap().to_rust_path();
        let out = derive_def(ptr, dt);
        map.get_mut(&path).unwrap().push(out);
    }
    let mut file = quote!(#![allow(
        unused_imports, // `use` above every mod
        clippy::unit_arg, // variant_num => Enum::Variant(val.into_unit()),
        clippy::let_unit_value, // Enum::Variant(val) => { let _ = val; Value::Unit },
        clippy::redundant_closure, // Value::List(Type::SimpleType, self.field.into_iter().map(|sv| Value::SimpleType(sv)).collect()),
        clippy::redundant_field_names, // (Type::Unknown) Struct { field: field }
        clippy::map_identity, // (Type::List(Type::Unknown)) val.into_list().into_iter().map(|sv| sv).collect(),
    )]);
    file.extend(map.into_iter().map(|(path, outs)| {
        let path = ident(path);
        quote!(
            pub mod #path {
                use crate::{types::*, meta::{Timestamp, ObjectPtr, TypePtr}};
                #(#outs)*
            }
        )
    }));
    file
}

const PATH: &str = "core/std/codegen.rs";
const HEADER: &[u8] = b"// This is a generated file. Do not modify, run `cargo run --bin schema-derive` to update.\n";

fn main() {
    use std::{fs::OpenOptions, env::args_os, io::{Read, Write}};
    let path = args_os().nth(1).unwrap_or_else(|| PATH.into());
    {
        let mut f = OpenOptions::new().read(true).open(&path).unwrap();
        let mut buf = vec![0; HEADER.len()];
        f.read_exact(&mut buf).unwrap();
        if buf != HEADER {
            panic!("overwrite protected");
        }
    }
    {
        let src = syn::parse2::<syn::File>(derive_file()).unwrap();
        let src = prettyplease::unparse(&src);
        let mut f = OpenOptions::new().write(true).truncate(true).open(&path).unwrap();
        f.write_all(HEADER).unwrap();
        f.write_all(src.as_bytes()).unwrap();
    }
}
