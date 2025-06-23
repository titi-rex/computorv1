use std::ops::{Add, Mul, Neg};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub enum Sign {
    #[default]
    Positive,
    Negative,
}

impl Sign {
    pub fn from(n: i8) -> Sign {
        if n >= 0 {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Sign::Positive {
            write!(f, "{}", "")
        } else {
            write!(f, "{}", "-")
        }
    }
}

impl Add for Sign {
    type Output = Sign;
    fn add(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }
}

impl Mul for Sign {
    type Output = Sign;
    fn mul(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

impl<T: Neg<Output = T>> Mul<T> for Sign {
    type Output = T;
    fn mul(self, rhs: T) -> Self::Output {
        match self {
            Sign::Positive => rhs,
            Sign::Negative => -rhs,
        }
    }
}
