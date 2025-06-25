use crate::complex::Complex;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Roots<T: PartialOrd, V: PartialOrd> {
    Unsolvable,
    Zero,
    Any,
    One(T),
    Two(T, T),
    Complex(V),
}

impl Display for Roots<f32, Complex> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Roots::Unsolvable => write!(f, "Can't solve"),
            Roots::Zero => write!(f, "Zero solutions"),
            Roots::Any => write!(f, "Any number is a solution"),
            Roots::One(r) => write!(f, "One real root: {}", r),
            Roots::Two(r1, r2) => write!(f, "Two real root: {} and {}", r1, r2),
            Roots::Complex(r) => write!(f, "Two complex roots: {} ± i{}", r.re(), r.im()),
        }
    }
}

