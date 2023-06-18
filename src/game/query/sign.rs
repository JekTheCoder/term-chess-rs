#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}

impl From<Sign> for isize {
    fn from(sign: Sign) -> Self {
        match sign {
            Sign::Positive => 1,
            Sign::Negative => -1,
            Sign::Zero => 0,
        }
    }
}

impl Sign {
    pub const fn into_isize(self) -> isize {
        match self {
            Self::Positive => 1,
            Self::Negative => -1,
            Self::Zero => 0,
        }
    }
}
