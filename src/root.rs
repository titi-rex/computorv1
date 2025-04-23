use crate::complex::Complex;
use crate::rational::Rational;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Roots<T: PartialOrd, V: PartialOrd> {
    Unsolvable,
    Zero,
    Any,
    One(T),
    Two(T, T),
    Complex(V, V),
}

impl<T: PartialOrd + Display, V: PartialOrd> Roots<T, V> {
    pub fn new_two(r1: T, r2: T) -> Roots<T, V> {
        if r1 <= r2 {
            Roots::Two(r1, r2)
        } else {
            Roots::Two(r2, r1)
        }
    }

    pub fn new_complex(r1: V, r2: V) -> Roots<T, V> {
        if r1 <= r2 {
            Roots::Complex(r1, r2)
        } else {
            Roots::Complex(r2, r1)
        }
    }
}

impl Display for Roots<Rational, Complex> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Roots::Unsolvable => write!(f, "Can't solve"),
            Roots::Zero => write!(f, "Zero solutions"),
            Roots::Any => write!(f, "Any number is a solution"),
            Roots::One(r) => write!(f, "One real root: {}", r),
            Roots::Two(r1, r2) => write!(f, "Two real root: {} and {}", r1, r2),
            Roots::Complex(r1, r2) => write!(f, "Two complex roots: {} and {}", r1, r2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complex::Complex;
    use crate::rational::Rational;
    use crate::sign::Sign;

    #[test]
    fn ordering() {
        ordering_rationals(
            Rational::new(Sign::Positive, 15, 2),
            Rational::new(Sign::Positive, 15, 2),
        );
        ordering_rationals(
            Rational::new(Sign::Positive, 15, 2),
            Rational::new(Sign::Negative, 15, 2),
        );
        ordering_complex(
            Complex::new(0.,0.),
            Complex::new(2.,10.),
        );
        ordering_complex(
            Complex::new(2.,10.),
            Complex::new(0.,0.),
        );
        
    }

    fn ordering_rationals(r1: Rational, r2: Rational) {
        if let Roots::Two(r1, r2) = Roots::<Rational, Complex>::new_two(r1, r2) {
            assert!(r1 <= r2)
        }
    }

    fn ordering_complex(r1: Complex, r2: Complex) {
        if let Roots::Two(r1, r2) = Roots::<Rational, Complex>::new_complex(r1, r2) {
            assert!(r1 <= r2)
        }
    }
}
