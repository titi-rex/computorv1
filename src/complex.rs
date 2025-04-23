use num::Zero;
use std::fmt::Display;

pub struct Complex {
    real: f32,
    imag: f32,
}

impl Complex {
    pub fn new(real: f32, imag: f32) -> Complex {
        Complex { real, imag }
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
