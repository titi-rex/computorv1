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

#[cfg(test)]
mod test {
    use crate::sign::Sign;


    #[test]
    fn creation(){
        let s1 = Sign::from(0);
        let s2 = Sign::from(2);
        let s3 = Sign::from(-5);
        let s4 = Sign::from(-1);

        assert!(s1 == Sign::Positive);
        assert!(s2 == Sign::Positive);
        assert!(s3 == Sign::Negative);
        assert!(s4 == Sign::Negative);
    }

    #[test]
    fn addition() {
        assert!(Sign::Positive + Sign::Positive == Sign::Positive);
        assert!(Sign::Negative + Sign::Negative == Sign::Positive);
        assert!(Sign::Positive + Sign::Negative == Sign::Negative);
    }

    #[test]
    fn multiplication() {
        assert!(Sign::Positive * Sign::Positive == Sign::Positive);
        assert!(Sign::Negative * Sign::Negative == Sign::Positive);
        assert!(Sign::Positive * Sign::Negative == Sign::Negative);
    }
}
