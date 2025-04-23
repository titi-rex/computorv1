use std::cmp::Ordering;
use crate::math::{gcd, number_len};
use crate::sign::Sign;
use num::{Float, One};
use std::fmt::Display;
use std::ops::{Add, Div, DivAssign, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rational {
    sign: Sign,
    numerator: u64,
    denominator: u64,
}



impl Rational {
    /// Create a new Rational, automatically reduce it
    pub fn new(sign: Sign, numerator: u64, denominator: u64) -> Self {
        let mut raw = Rational {
            sign,
            numerator,
            denominator,
        };
        if denominator != 1 {
            raw.reduce();
        }
        raw
    }

    /// Converts a float into a Rational
    pub fn from_f32(f: f32) -> Option<Rational> {
        if !f.is_finite() {
            return None;
        }
        if f == 0. {
            return Some(Rational::new(Sign::Positive, 0, 1));
        }
        let (mantissa, exponent, sign) = f.integer_decode();
        let ratio_sign = if sign == 1 {
            Sign::Positive
        } else {
            Sign::Negative
        };
        if exponent < 0 {
            let denom = u64::one() << ((-exponent) as usize);
            Some(Rational::new(ratio_sign, mantissa, denom))
        } else {
            let mut numer = mantissa;
            numer <<= exponent as usize;
            Some(Rational::new(ratio_sign, mantissa, numer))
        }
    }

    pub fn from_f32_couple(num: f32, denom: f32) -> Option<Rational> {
        let mut r = Rational::from_f32(num)? / Rational::from_f32(denom)?;
        r.reduce();
        Some(r)
    }

    /// Get the sign
    pub fn sign(&self) -> &Sign {
        &self.sign
    }

    /// Get the numerator
    pub fn numerator(&self) -> u64 {
        self.numerator
    }

    /// Get the denominator
    pub fn denominator(&self) -> u64 {
        self.denominator
    }

    /// Compute the Rational as a float
    pub fn compute(&self) -> f32 {
        let tmp = self.sign * (self.numerator as f32 / self.denominator as f32);
        (tmp * 100000.0).round() / 100000.0
    }

    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    /// Reduce the Rational as an irreducible fraction if possible
    pub fn reduce(&mut self) {
        let g = gcd(self.numerator, self.denominator);
        if g > 1 {
            *self /= g;
        }
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if number_len(self.numerator) >= 5 {
            write!(f, "{}", self.compute())
        } else if self.denominator() == 1 {
            write!(f, "{}", self.numerator())
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.sign == other.sign {
            let a = self.numerator * other.denominator;
            let b = self.denominator * other.numerator;
            Some(a.cmp(&b))
        } else if self.sign == Sign::Positive {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Add for Rational {
    type Output = Rational;
    fn add(self, rhs: Rational) -> Self::Output {
        let numerator = self.sign * (self.numerator * rhs.denominator) as i128
            + (self.denominator * rhs.numerator) as i128;

        Rational::new(
            Sign::from(numerator.signum() as i8),
            numerator.abs() as u64,
            self.denominator * rhs.denominator,
        )
    }
}

impl Sub for Rational {
    type Output = Rational;
    fn sub(self, rhs: Rational) -> Self::Output {
        let numerator = self.sign * (self.numerator * rhs.denominator) as i128
            - (self.denominator * rhs.numerator) as i128;

        Rational::new(
            Sign::from(numerator.signum() as i8),
            numerator.abs() as u64,
            self.denominator * rhs.denominator,
        )
    }
}

impl Mul for Rational {
    type Output = Rational;
    fn mul(self, rhs: Rational) -> Self::Output {
        Rational::new(
            self.sign * rhs.sign,
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl Div for Rational {
    type Output = Rational;
    fn div(self, rhs: Rational) -> Self::Output {
        Rational::new(
            self.sign * rhs.sign,
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl DivAssign<u64> for Rational {
    fn div_assign(&mut self, rhs: u64) {
        self.numerator /= rhs;
        self.denominator /= rhs;
    }
}

impl DivAssign<u32> for Rational {
    fn div_assign(&mut self, rhs: u32) {
        self.numerator /= rhs as u64;
        self.denominator /= rhs as u64;
    }
}

#[cfg(test)]
mod tests {
    use crate::rational::Rational;
    use crate::sign::Sign;

    #[test]
    fn new() {
        assert_eq!(
            Rational::new(Sign::Negative, 1, 2),
            Rational {
                sign: Sign::Negative,
                numerator: 1,
                denominator: 2
            },
        );

        assert_eq!(
            Rational::new(Sign::Positive, 5, 2),
            Rational {
                sign: Sign::Positive,
                numerator: 5,
                denominator: 2
            },
        );
    }

    #[test]
    fn reduce() {
        assert_eq!(
            Rational::new(Sign::Positive, 10, 2),
            Rational {
                sign: Sign::Positive,
                numerator: 5,
                denominator: 1
            },
        );

        assert_eq!(
            Rational::new(Sign::Positive, 6, 2),
            Rational {
                sign: Sign::Positive,
                numerator: 3,
                denominator: 1
            },
        );

        assert_eq!(
            Rational::new(Sign::Positive, 50, 20),
            Rational {
                sign: Sign::Positive,
                numerator: 5,
                denominator: 2
            },
        );
    }

    #[test]
    fn from_f32() {
        assert_eq!(
            Rational::from_f32(1.5).expect("Should not be Nan"),
            Rational::new(Sign::Positive, 3, 2)
        );

        assert_eq!(
            Rational::from_f32(2.5).expect("Should not be Nan"),
            Rational::new(Sign::Positive, 5, 2)
        );

        assert_eq!(
            Rational::from_f32(-0.5).expect("Should not be Nan"),
            Rational::new(Sign::Negative, 1, 2)
        );

        assert_eq!(
            Rational::from_f32(-4.75).expect("Should not be Nan"),
            Rational::new(Sign::Negative, 19, 4)
        );
    }

    #[test]
    #[should_panic]
    fn from_f32_nan() {
        let _r = Rational::from_f32(f32::NAN).unwrap();
    }
}
