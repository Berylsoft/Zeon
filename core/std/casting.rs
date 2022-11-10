use super::codegen;
use crate::types::*;

impl codegen::meta::Typeptr {
    pub fn from_local(local: TypePtr) -> Self {
        match local {
            TypePtr::Std(stdptr) => Self::Std(codegen::meta::TypeptrStd(stdptr.to_u16() as u64)),
            TypePtr::Hash(hash) => Self::Hash(codegen::meta::TypeptrHash(crate::util::typehash_to_u64(hash))),
        }
    }

    pub fn to_local(self) -> TypePtr {
        match self {
            Self::Std(codegen::meta::TypeptrStd(n)) => TypePtr::from_u16(n as u16),
            Self::Hash(codegen::meta::TypeptrHash(n)) => TypePtr::Hash(crate::util::u64_to_typehash(n)),
        }
    }
}
