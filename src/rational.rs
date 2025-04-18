use std::fmt::Display;
use std::ops::Add;
use crate::math::gcd;

pub struct Rational {
    numerator: f32,
    denominator: f32,
}

impl Rational {
    pub fn new(numerator: f32, denominator: f32) -> Self {
        let mut raw = Rational { numerator, denominator };
        if denominator == 1. {
            raw
        } else {
            raw.reduce()
        }
    }

    pub fn numerator(&self) -> f32 {
        self.numerator
    }

    pub fn denominator(&self) -> f32 {
        self.denominator
    }

    pub fn compute(&self) -> f32 {
        self.numerator/self.denominator
    }

    pub fn reduce(&self) -> Rational {
        let g = gcd(&self.numerator, &self.denominator);
        dbg!("gcd: {:?}", g);
        if g > 1. {
            Self {
                numerator:  self.numerator/g,
                denominator:  self.denominator/g,
            }
        } else {
            Self {
                ..*self
            }
        }
    }

}


impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator == 1. {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }

}

impl Add for Rational {
    type Output = Rational;
    fn add(self, rhs: Rational) -> Self::Output {
        Rational::new(self.numerator + rhs.numerator, self.denominator + rhs.denominator)
    }
}

