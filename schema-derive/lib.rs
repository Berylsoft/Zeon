#[allow(dead_code)]

use proc_macro2::TokenStream;
use quote::quote;
use zeon::types::{Type, TypePtr};

fn ptr2tokens(ptr: TypePtr) -> TokenStream {
    match ptr {
        TypePtr::Std(ptr) => {
            let n = ptr.as_u16();
            quote!(::zeon::types::TypePtr::Std(::zeon::types::StdPtr(#n)))
        },
        TypePtr::Hash(hash) => {
            let hex = hex::encode(hash);
            quote!(::zeon::types::TypePtr::Hash(::hex_literal::hex!(#hex)))
        },
    }
}

fn type2tokens(ty: Type) -> TokenStream {
    match ty {
        Type::Unit => quote!(::zeon::types::Type::Unit),
        Type::Bool => quote!(::zeon::types::Type::Bool),
        Type::Int => quote!(::zeon::types::Type::Int),
        Type::UInt => quote!(::zeon::types::Type::UInt),
        Type::Float => quote!(::zeon::types::Type::Float),
        Type::String => quote!(::zeon::types::Type::String),
        Type::Bytes => quote!(::zeon::types::Type::Bytes),
        Type::Option(sty) => {
            let stokens = type2tokens(*sty);
            quote!(::zeon::types::Type::Option(::std::boxed::Box::new(#stokens)))
        },
        Type::List(sty) => {
            let stokens = type2tokens(*sty);
            quote!(::zeon::types::Type::List(::std::boxed::Box::new(#stokens)))
        },
        Type::Map(styk, styv) => {
            let stokensk = type2tokens(*styk);
            let stokensv = type2tokens(*styv);
            quote!(::zeon::types::Type::Map(::std::boxed::Box::new(#stokensk), ::std::boxed::Box::new(#stokensv)))
        },
        Type::Alias(ptr) => {
            let ptrtokens = ptr2tokens(ptr);
            quote!(::zeon::types::Type::Alias(#ptrtokens))
        },
        Type::Enum(ptr) => {
            let ptrtokens = ptr2tokens(ptr);
            quote!(::zeon::types::Type::Enum(#ptrtokens))
        },
        Type::Tuple(ptr) => {
            let ptrtokens = ptr2tokens(ptr);
            quote!(::zeon::types::Type::Tuple(#ptrtokens))
        },
        Type::Struct(ptr) => {
            let ptrtokens = ptr2tokens(ptr);
            quote!(::zeon::types::Type::Struct(#ptrtokens))
        },
        Type::Type => quote!(::zeon::types::Type::Type),
        Type::ObjectRef => quote!(::zeon::types::Type::ObjectRef),
    }
}

fn ptr2rustpath(ptr: TypePtr) -> TokenStream {
    let ptr = ptr.as_std_inner().unwrap();
    // ::zeon_std::RUSTPATH_MAP.get(ptr).unwrap()
    quote!(PTR2RUSTPATH_TODO(#ptr))
}

fn type2type(ty: Type) -> TokenStream {
    match ty {
        Type::Unit => quote!(()),
        Type::Bool => quote!(bool),
        Type::Int => quote!(i64),
        Type::UInt => quote!(u64),
        Type::Float => quote!(f64),
        Type::String => quote!(::std::string::String),
        Type::Bytes => quote!(::std::vec::Vec<u8>),
        Type::Option(sty) => {
            let sty = type2type(*sty);
            quote!(::std::option::Option<#sty>)
        },
        Type::List(sty) => {
            let sty = type2type(*sty);
            quote!(::std::vec::Vec<#sty>)
        },
        Type::Map(styk, styv) => {
            let styk = type2type(*styk);
            let styv = type2type(*styv);
            quote!(::std::vec::Vec<(#styk, #styv)>)
        },
        Type::Alias(ptr) |
        Type::Enum(ptr) |
        Type::Tuple(ptr) |
        Type::Struct(ptr) => {
            let rustpath = ptr2rustpath(ptr);
            rustpath
        },
        Type::Type => quote!(::zeon::types::Type),
        Type::ObjectRef => quote!(::zeon::metadata::ObjectRef),
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
        Type::Option(sty) => {
            let sde = type2de(*sty, quote!(sv));
            quote!(#v.into_option()opt.map(|sv| #sde))
        },
        Type::List(sty) => {
            let sde = type2de(*sty, quote!(sv));
            quote!(#v.into_list().into_iter().map(|sv| #sde).collect())
        },
        Type::Map(styk, styv) => {
            let sdek = type2de(*styk, quote!(sk));
            let sdev = type2de(*styv, quote!(sv));
            quote!(#v.into_map().into_iter().map(|(sk, sv)| (#sdek, #sdev)).collect())
        },
        Type::Alias(_ptr) |
        Type::Enum(_ptr) |
        Type::Tuple(_ptr) |
        Type::Struct(_ptr) => quote!(#v.deserialize_into()),
        Type::Type => quote!(#v.into_type()),
        Type::ObjectRef => quote!(#v.into_objectref()),
    }
}

fn type2ser(ty: Type, v: TokenStream) -> TokenStream {
    match ty {
        Type::Unit => quote!(::zeon::types::Value::Unit),
        Type::Bool => quote!(::zeon::types::Value::Bool(#v)),
        Type::Int => quote!(::zeon::types::Value::Int(#v)),
        Type::UInt => quote!(::zeon::types::Value::UInt(#v)),
        Type::Float => quote!(::zeon::types::Value::from_float(#v)),
        Type::String => quote!(::zeon::types::Value::String(#v)),
        Type::Bytes => quote!(::zeon::types::Value::Bytes(#v)),
        Type::Option(sty) => {
            let sty = *sty;
            let sser = type2ser(sty, quote!(sv));
            quote!(::zeon::types::Value::Option(sty, #v.map(|sv| #sser)))
        },
        Type::List(sty) => {
            let stytokens = type2tokens(*sty.clone());
            let sser = type2ser(*sty, quote!(sv));
            quote!(::zeon::types::Value::List(#stytokens, #v.into_iter().map(|sv| #sser).collect()))
        },
        Type::Map(styk, styv) => {
            let stytokensk = type2tokens(*styk.clone());
            let stytokensv = type2tokens(*styv.clone());
            let sserk = type2ser(*styk, quote!(sk));
            let sserv = type2ser(*styv, quote!(sv));
            quote!(::zeon::types::Value::Map((#stytokensk, #stytokensv), #v.into_iter().map(|(sk, sv)| (#sserk, #sserv)).collect()))
        },
        Type::Alias(ptr) => {
            let ptrtoken = ptr2tokens(ptr);
            quote!(::zeon::types::Value::Alias(#ptrtoken, #v.serialize()))
        },
        Type::Enum(ptr) => {
            let ptrtoken = ptr2tokens(ptr);
            quote!(::zeon::types::Value::Enum(#ptrtoken, #v.serialize()))
        },
        Type::Tuple(ptr) => {
            let ptrtoken = ptr2tokens(ptr);
            quote!(::zeon::types::Value::Tuple(#ptrtoken, #v.serialize()))
        },
        Type::Struct(ptr) => {
            let ptrtoken = ptr2tokens(ptr);
            quote!(::zeon::types::Value::Struct(#ptrtoken, #v.serialize()))
        },
        Type::Type => quote!(::zeon::types::Value::Type(#v)),
        Type::ObjectRef => quote!(::zeon::types::Value::ObjectRef(#v)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let ty = Type::List(Box::new(Type::Alias(TypePtr::from_u16(4185))));
        println!("{}", type2type(ty).to_string())
    }
}
