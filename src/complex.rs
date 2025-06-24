use crate::math::sqrt;
use crate::rational::Rational;
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Complex {
    real: Rational,
    imag: Rational,
}

impl Complex {
    pub fn new(real: f32, imag: f32) -> Complex {
        Complex {
            real: Rational::from_f32(real).unwrap(),
            imag: Rational::from_f32(imag).unwrap(),
        }
    }

    pub fn from_rational(real: Rational, imag: Rational) -> Complex {
        Complex {
            real: real,
            imag: imag,
        }
    }

    pub fn modulus(&self) -> f32 {
        let re = self.real.compute();
        let im = self.imag.compute();
        sqrt(re * re + im + im)
    }

    pub fn re(&self) -> &Rational {
        &self.real
    }

    pub fn im(&self) -> &Rational {
        &self.imag
    }
}

impl PartialOrd for Complex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.modulus() == other.modulus() {
            Some(Ordering::Equal)
        } else if self.modulus() >= other.modulus() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if self.real.is_zero() == false {
            s += format!("{}", self.real).as_str();
        }
        if self.imag.is_zero() == false {
            s += format!(" {}i", self.real).as_str();
        }
        write!(f, "{}", s)
    }
}
