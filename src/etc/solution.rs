use std::fmt::{Display, Formatter, Result};
use Solution::*;

#[derive(Clone)]
pub enum Solution {
    I32(i32),
    U32(u32),
    USize(usize),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I32(x) => x.fmt(f),
            U32(x) => x.fmt(f),
            USize(x) => x.fmt(f),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    }
}

impl_from!(i32, I32);
impl_from!(u32, U32);
impl_from!(usize, USize);
