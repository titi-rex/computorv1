use crate::rational::Rational;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
