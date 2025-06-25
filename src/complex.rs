use crate::math::sqrt;
use std::cmp::Ordering;
use std::fmt::Display;
use num::Zero;

#[derive(Debug, Clone, PartialEq)]
pub struct Complex {
    real: f32,
    imag: f32,
}

impl Complex {
    pub fn new(real: f32, imag: f32) -> Complex {
        Complex {
            real,
            imag,
        }
    }
    
    pub fn modulus(&self) -> f32 {
        sqrt(self.real * self.real + self.imag * self.imag)
    }

    pub fn re(&self) -> &f32 {
        &self.real
    }

    pub fn im(&self) -> &f32 {
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
