use super::codegen;

#[derive(Clone, Copy, Debug)]
pub struct StdCheckError {
    pub msg: &'static str,
}

pub trait StdCheck<T>: Sized + From<T> + Into<T> {
    fn check(inner: &T) -> Result<(), StdCheckError>;

    fn new(inner: T) -> Result<Self, StdCheckError> {
        Self::check(&inner)?;
        Ok(Self::from(inner))
    }

    // fn self_check(self) -> Result<Self, StdCheckError> {
    //     let inner = self.into();
    //     Self::check(&inner)?;
    //     Ok(Self::from(inner))
    // }
}

impl StdCheck<String> for codegen::prim::SimpleName {
    fn check(inner: &String) -> Result<(), StdCheckError> {
        if !inner.is_ascii() {
            return Err(StdCheckError { msg: "std:prim:simple-name is not ascii" });
        }
        for b in inner.as_bytes() {
            if !matches!(b, b'0'..=b'9' | b'a'..=b'z' | b'-') {
                return Err(StdCheckError { msg: "std:prim:simple-name is not 0-9 | a-z | '-'" });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert!(codegen::prim::SimpleName::check(&"inner".to_owned()).is_ok());
        assert!(codegen::prim::SimpleName::check(&"inNer".to_owned()).is_err());
        assert!(codegen::prim::SimpleName::check(&"inN_er".to_owned()).is_err());
        assert!(codegen::prim::SimpleName::check(&"inn-er".to_owned()).is_ok());
    }
}
