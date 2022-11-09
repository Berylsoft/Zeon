use proc_macro2::TokenStream;
use quote::quote;
use zeon::{types::{Type, TypePtr, DefType}, std::DEFTYPES, util::{to_pascal_case, to_snake_case}};

fn ident<S: AsRef<str>>(s: S) -> TokenStream {
    s.as_ref().parse().unwrap()
}

fn ptr2rustpath(ptr: TypePtr) -> TokenStream {
    let ptr = ptr.as_std_inner().unwrap();
    let path = zeon::std::ptr2path(ptr).unwrap();
    let path = ident(path.to_rustpath());
    quote!(#path)
}

fn ptr2rustname(ptr: TypePtr) -> TokenStream {
    let ptr = ptr.as_std_inner().unwrap();
    let path = zeon::std::ptr2path(ptr).unwrap();
    let path = ident(path.to_rustname());
    quote!(#path)
}

fn ptr2tokens(ptr: TypePtr) -> TokenStream {
    match ptr {
        TypePtr::Std(ptr) => {
            let n = ptr.as_u16();
            quote!(TypePtr::from_u16_unchecked(#n))
        },
        TypePtr::Hash(hash) => {
            let hex = hex::encode(hash);
            quote!(TypePtr::Hash(hex_literal::hex!(#hex)))
        },
    }
}

fn type2tokens(ty: Type) -> TokenStream {
    match ty {
        Type::Unit => quote!(Type::Unit),
        Type::Bool => quote!(Type::Bool),
        Type::Int => quote!(Type::Int),
        Type::UInt => quote!(Type::UInt),
        Type::Float => quote!(Type::Float),
        Type::String => quote!(Type::String),
        Type::Bytes => quote!(Type::Bytes),
        Type::Type => quote!(Type::Type),
        Type::ObjectRef => quote!(Type::ObjectRef),

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
        Type::Unit => quote!(()),
        Type::Bool => quote!(bool),
        Type::Int => quote!(i64),
        Type::UInt => quote!(u64),
        Type::Float => quote!(f64),
        Type::String => quote!(String),
        Type::Bytes => quote!(Vec<u8>),
        Type::Type => quote!(Type),
        Type::ObjectRef => quote!(ObjectRef),

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
        Type::Enum(ptr) |
        Type::Struct(ptr) => {
            let rustpath = ptr2rustpath(ptr);
            rustpath
        },
    }
}

fn type2de(ty: Type, v: TokenStream) -> TokenStream {
    match ty {
        Type::Unit => quote!(#v.into_unit()),
        Type::Bool => quote!(#v.into_bool()),
        Type::Int => quote!(#v.into_int()),
        Type::UInt => quote!(#v.into_uint()),
        Type::Float => quote!(#v.into_float()),
        Type::String => quote!(#v.into_string()),
        Type::Bytes => quote!(#v.into_bytes()),
        Type::Type => quote!(#v.into_type()),
        Type::ObjectRef => quote!(#v.into_objectref()),

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

        Type::Alias(_ptr) |
        Type::Enum(_ptr) |
        Type::Struct(_ptr) => quote!(#v.deserialize_into()),
    }
}

fn type2ser(ty: Type, v: TokenStream) -> TokenStream {
    match ty {
        Type::Unit => quote!(Value::Unit),
        Type::Bool => quote!(Value::Bool(#v)),
        Type::Int => quote!(Value::Int(#v)),
        Type::UInt => quote!(Value::UInt(#v)),
        Type::Float => quote!(Value::from_float(#v)),
        Type::String => quote!(Value::String(#v)),
        Type::Bytes => quote!(Value::Bytes(#v)),
        Type::Type => quote!(Value::Type(#v)),
        Type::ObjectRef => quote!(Value::ObjectRef(#v)),

        Type::Option(sty) => {
            let sty_ty = type2tokens(*sty.clone());
            let sty = type2ser(*sty, quote!(sv));
            quote!(Value::Option(#sty_ty, #v.map(|sv| #sty)))
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

        Type::Alias(ptr) => {
            let ptr = ptr2tokens(ptr);
            quote!(Value::Alias(#ptr, Box::new(#v.serialize())))
        },
        Type::Enum(ptr) => {
            let ptr = ptr2tokens(ptr);
            quote!(Value::Enum(#ptr, 0, Box::new(#v.serialize())))
        },
        Type::Struct(ptr) => {
            let ptr = ptr2tokens(ptr);
            quote!(Value::Struct(#ptr, Box::new(#v.serialize())))
        },
    }
}

fn derive_def(ptr: u16, dt: DefType) -> TokenStream {
    match dt {
        DefType::Alias(ty) => {
            // let path = ptr2rustpath(TypePtr::from_u16_unchecked(ptr));
            let name = ptr2rustname(TypePtr::from_u16_unchecked(ptr));
            let inner_ty = type2type(ty.clone());
            let inner_ser = type2ser(ty.clone(), quote!(self.inner));
            let inner_de = type2de(ty.clone(), quote!(val));
            quote!(
                pub struct #name {
                    pub inner: #inner_ty,
                }

                impl Schema for #name {
                    const PTR: TypePtr = TypePtr::from_u16_unchecked(#ptr);

                    fn serialize(self) -> Value {
                        #inner_ser
                    }

                    fn deserialize(val: Value) -> Self {
                        Self { inner: #inner_de }
                    }
                }
            )
        },
        DefType::Enum(tys) => quote!(),
        DefType::Struct(fields) => {
            let name = ptr2rustname(TypePtr::from_u16_unchecked(ptr));
            let (names, tys): (Vec<String>, Vec<Type>) = fields.clone().into_iter().unzip();
            let names = names.into_iter().map(|name| ident(to_snake_case(&name)));
            let names2 = names.clone();
            let tyts = tys.clone().into_iter().map(type2type);
            let sers = fields.into_iter().map(|(name, ty)| type2ser(ty, ident(macros::concat_string!("self.", to_snake_case(&name)))));
            let des = tys.into_iter().enumerate().map(|(i, ty)| type2de(ty, quote!(val[#i])));
            
            quote!(
                pub struct #name {
                    #(pub #names: #tyts,)*
                }

                impl Schema for #name {
                    const PTR: TypePtr = TypePtr::from_u16_unchecked(#ptr);

                    fn serialize(self) -> Value {
                        Value::Struct(TypePtr::from_u16_unchecked(#ptr), vec![
                            #(#sers,)*
                        ])
                    }

                    fn deserialize(val: Value) -> Self {
                        let val = val.into_struct();
                        Self {
                            #(#names2: #des,)*
                        }
                    }
                }
            )
        },
    }
}

fn prelude() -> TokenStream {
    quote!(
        #![allow(non_camel_case_types)]
        use crate::types::*;
    )
}

fn main() {
    // let ty = Type::List(Box::new(Type::Alias(TypePtr::from_u16(0x0001))));
    // let ty = Type::Tuple(vec![Type::UInt, Type::Unit]);
    // println!("{}", type2ser(ty, quote!(v)).to_string());
    println!("{}", prelude());
    for (ptr, dt) in DEFTYPES.iter() {
        println!("{}", derive_def(*ptr, dt.clone()));
    }
}
