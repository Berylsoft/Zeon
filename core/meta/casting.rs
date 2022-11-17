use super::*;

impl TypePtr {
    pub const fn from_u16(n: u16) -> TypePtr {
        assert!(crate::util::check_stdptr(n));
        TypePtr::Std(StdPtr(n))
    }

    pub const fn from_u16_unchecked(n: u16) -> TypePtr {
        TypePtr::Std(StdPtr(n))
    }

    pub fn from_path(path: &str) -> TypePtr {
        TypePtr::Hash(crate::util::shake256(path.as_bytes()))
    }

    pub const fn as_std(self) -> Option<StdPtr> {
        match self {
            Self::Std(stdptr) => Some(stdptr),
            _ => None,
        }
    }

    pub const fn as_std_inner(self) -> Option<u16> {
        match self {
            Self::Std(StdPtr(n)) => Some(n),
            _ => None,
        }
    }

    pub const fn as_hash(self) -> Option<[u8; 7]> {
        match self {
            Self::Hash(hash) => Some(hash),
            _ => None,
        }
    }
}

impl StdPtr {
    pub const fn from_u16(n: u16) -> StdPtr {
        assert!(crate::util::check_stdptr(n));
        StdPtr(n)
    }

    #[inline]
    pub const fn from_u16_unchecked(n: u16) -> StdPtr {
        StdPtr(n)
    }

    #[inline]
    pub const fn to_u16(self) -> u16 {
        self.0
    }
}

impl TypePtr {
    pub const SIZE: usize = 8;

    pub fn from_bytes(raw: [u8; Self::SIZE]) -> Self {
        match raw[0] {
            0x00 => TypePtr::Std(StdPtr::from_u16(u16::from_be_bytes(raw[6..7].try_into().unwrap()))),
            0xFF => TypePtr::Hash(raw[1..7].try_into().unwrap()),
            _ => unreachable!(),
        }
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        match self {
            TypePtr::Std(stdptr) => {
                buf[6..8].copy_from_slice(&stdptr.to_u16().to_be_bytes());
            },
            TypePtr::Hash(hash) => {
                buf[0] = 0xFF;
                buf[1..8].copy_from_slice(hash);
            }
        }
        buf
    }
}

impl From<u8> for RevType {
    fn from(b: u8) -> Self {
        match b {
            0 => RevType::Const,
            1 => RevType::Mut,
            2 => RevType::IterListAdd,
            3 => RevType::IterSetAdd,
            4 => RevType::IterSetRemove,
            5 => RevType::Complex,
            _ => unreachable!(),
        }
    }
}

impl From<RevType> for u8 {
    fn from(s: RevType) -> Self {
        match s {
            RevType::Const => 0,
            RevType::Mut => 1,
            RevType::IterListAdd => 2,
            RevType::IterSetAdd => 3,
            RevType::IterSetRemove => 4,
            RevType::Complex => 5,
        }
    }
}

// region: TODO macro

impl RevPtr {
    pub const SIZE: usize = ObjectPtr::SIZE + TypePtr::SIZE + 1 + 1;

    pub fn from_bytes(raw: [u8; Self::SIZE]) -> Self {
        let mut offset: usize = 0;
        let object = ObjectPtr::from_bytes(raw[offset..(offset + ObjectPtr::SIZE)].try_into().unwrap());
        offset += ObjectPtr::SIZE;
        let trait_type = TypePtr::from_bytes(raw[offset..(offset + TypePtr::SIZE)].try_into().unwrap());
        offset += TypePtr::SIZE;
        let [attr]: [u8; 1] = raw[offset..offset + 1].try_into().unwrap();
        offset += 1;
        let [rev_type]: [u8; 1] = raw[offset..offset + 1].try_into().unwrap();
        offset += 1;
        assert_eq!(offset, Self::SIZE);
        Self { object, trait_type, attr, rev_type: rev_type.into() }
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        let mut offset: usize = 0;
        buf[offset..ObjectPtr::SIZE].copy_from_slice(&self.object.to_bytes());
        offset += ObjectPtr::SIZE;
        buf[offset..offset + TypePtr::SIZE].copy_from_slice(&self.trait_type.to_bytes());
        offset += TypePtr::SIZE;
        buf[offset..offset + 1].copy_from_slice(&[self.attr]);
        offset += 1;
        buf[offset..offset + 1].copy_from_slice(&[self.rev_type.into()]);
        offset += 1;
        assert_eq!(offset, Self::SIZE);
        buf
    }
}

impl CommitPtr {
    pub const SIZE: usize = Timestamp::SIZE + ObjectPtr::SIZE + 2;
    
    pub fn from_bytes(raw: [u8; Self::SIZE]) -> Self {
        let mut offset: usize = 0;
        let ts = Timestamp::from_bytes(raw[offset..(offset + Timestamp::SIZE)].try_into().unwrap());
        offset += Timestamp::SIZE;
        let opr = ObjectPtr::from_bytes(raw[offset..(offset + ObjectPtr::SIZE)].try_into().unwrap());
        offset += ObjectPtr::SIZE;
        let seq = u16::from_be_bytes(raw[offset..offset + 2].try_into().unwrap());
        offset += 2;
        assert_eq!(offset, Self::SIZE);
        Self { ts, opr, seq }
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        let mut offset: usize = 0;
        buf[offset..Timestamp::SIZE].copy_from_slice(&self.ts.to_bytes());
        offset += Timestamp::SIZE;
        buf[offset..offset + ObjectPtr::SIZE].copy_from_slice(&self.opr.to_bytes());
        offset += ObjectPtr::SIZE;
        buf[offset..offset + 2].copy_from_slice(&self.seq.to_be_bytes());
        offset += 2;
        assert_eq!(offset, Self::SIZE);
        buf
    }
}

// endregion

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(RevPtr::SIZE, 20);
        assert_eq!(CommitPtr::SIZE, 24);
    }
}
