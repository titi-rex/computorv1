use crate::complex::Complex;
use crate::math::sqrt;
use crate::rational::Rational;
use crate::root::Roots;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::num::ParseFloatError;
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct Polynomial(pub HashMap<i32, f32>);

impl Polynomial {
    const ERR_NAN_POLY: &'static str = "Polynomial shouldn't contain NAN coefficients";
    pub fn new() -> Polynomial {
        Polynomial(HashMap::new())
    }

    pub fn insert(&mut self, key: i32, value: f32) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: i32) -> &f32 {
        self.0.get(&key).or(Some(&0.0)).unwrap()
    }

    pub fn from_vec(v: &Vec<f32>) -> Polynomial {
        let mut polynomial = Polynomial::new();
        let mut exponent = 0;

        for c in v {
            polynomial.insert(exponent, *c);
            exponent += 1;
        }
        polynomial
    }

    pub fn from_str(raw: &str) -> Result<Polynomial, ParseFloatError> {
        let split = raw.split(" ").collect::<Vec<&str>>();
        let mut poly = Polynomial::new();
        let mut sign = 1.;
        let mut exponent = 0;

        for p in split.iter() {
            if p.is_empty() {
                continue;
            } else if p.to_string() == "-" {
                sign = -1.;
            } else if p.to_string() == "+" {
                sign = 1.;
            } else if p.to_string() == "*" {
                exponent += 1;
            } else if p.to_lowercase().contains("x^") {
                continue;
            } else {
                match p.parse::<f32>() {
                    Ok(v) => poly.insert(exponent, sign * v),
                    Err(error) => return Err(error),
                };
            }
        }
        Ok(poly)
    }

    pub fn degree(&self) -> i32 {
        let d = self.0.len() as i32 - 1;
        if d < 0 {
            0
        } else {
            d
        }
    }

    pub fn discriminant(p: &Polynomial) -> f32 {
        p.get(1) * p.get(1) - 4. * p.get(2) * p.get(0)
    }

    /// Search roots
    pub fn solve_roots(&self) -> Roots<Rational, Complex> {
        match self.degree() {
            0 if *self.get(0) == 0. => Roots::Any,
            0 if *self.get(0) != 0. => Roots::Zero,
            1 => Self::solve_affine(self),
            2 => Self::solve_quadratic(self),
            _ => Roots::Unsolvable,
        }
    }

    fn solve_affine(p: &Polynomial) -> Roots<Rational, Complex> {
        Roots::One(Rational::from_f32_couple(-p.get(0), *p.get(1)).expect(Polynomial::ERR_NAN_POLY))
    }

    fn solve_quadratic(p: &Polynomial) -> Roots<Rational, Complex> {
        let a = p.get(2);
        let b = p.get(1);
        match Polynomial::discriminant(p) {
            d if d > 0.0 => Roots::new_two(
                Rational::from_f32_couple(-b - sqrt(d), 2. * a)
                    .expect(Polynomial::ERR_NAN_POLY),
                Rational::from_f32_couple(-b + sqrt(d), 2. * a)
                    .expect(Polynomial::ERR_NAN_POLY),
            ),
            d if d < 0.0 => Roots::Complex(
                Complex::from_rational(
                    Rational::from_f32_couple(-b, 2. * a).expect(Polynomial::ERR_NAN_POLY),
                    Rational::from_f32_couple(sqrt(-d), 2. * a).expect(Polynomial::ERR_NAN_POLY)
                )
            ),
            _ => Roots::One(
                Rational::from_f32_couple(-b, 2. * a)
                    .expect(Polynomial::ERR_NAN_POLY),
            ),
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0.is_empty() {
            write!(f, "0")
        } else {
            let mut s = String::new();
            for exponent in self.0.keys().sorted() {
                if exponent
                    != self
                        .0
                        .keys()
                        .min()
                        .expect("Polynomial exponent should not be empty")
                {
                    s.push_str(&match *self.get(*exponent) < 0f32 {
                        true => " ",
                        false => " + ",
                    });
                }
                s.push_str(&match exponent {
                    e if *e == 0 => format!("{}", self.get(*exponent)),
                    e if *e == 1 => format!("{} * X", self.get(*exponent)),
                    _ => format!("{} * X^{}", self.get(*exponent), exponent),
                });
            }
            write!(f, "{}", s)
        }
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Polynomial) -> bool {
        self.0 == other.0
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (exponent, value) in rhs.0.iter() {
            let v = result.get(*exponent) + *value;
            if v != f32::default() {
                result.0.insert(*exponent, v);
            } else {
                result.0.remove(exponent);
            }
        }
        result
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (exponent, value) in rhs.0.iter() {
            let v = result.get(*exponent) - *value;
            if v != f32::default() {
                result.0.insert(*exponent, v);
            } else {
                result.0.remove(exponent);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::polynomial::Polynomial;
    use crate::root::Roots;

    #[test]
    fn creation() {
        let _p = Polynomial::new();
        let _p = Polynomial::from_str("").expect("should create polynomial zero");
        let _p = Polynomial::from_str("1").expect("should create polynomial 1");
    }

    #[test]
    fn degree() {
        assert_eq!(Polynomial::from_vec(&vec![1.]).degree(), 0);
        assert_eq!(Polynomial::from_vec(&vec![1., 1.]).degree(), 1);
        assert_eq!(Polynomial::from_vec(&vec![1., 1., 1.]).degree(), 2);
        assert_eq!(Polynomial::from_vec(&vec![1., 1., 1., 1.]).degree(), 3);
        assert_eq!(Polynomial::from_vec(&vec![1., 1., 1., 1., 1.]).degree(), 4);
    }

    #[test]
    fn equality() {
        assert_eq!(Polynomial::from_str("1"), Polynomial::from_str("1"));
    }

    #[test]
    fn addition() {
        let p1 = Polynomial::from_vec(&vec![1., 2., 3.]);
        let p2 = Polynomial::from_vec(&vec![1., 2., 3.]);
        let expected = Polynomial::from_vec(&vec![2., 4., 6.]);

        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn substraction() {
        let p1 = Polynomial::from_vec(&vec![1., 2., 3.]);
        let p2 = Polynomial::from_vec(&vec![1., 2., 3.]);
        let expected = Polynomial::from_vec(&vec![]);

        assert_eq!(p1 - p2, expected);
    }

    #[test] //
    fn find_roots() {
        assert_eq!(Polynomial::from_vec(&vec![1.]).solve_roots(), Roots::Zero);
        assert_eq!(Polynomial::from_vec(&vec![0.]).solve_roots(), Roots::Any);
        // assert_eq!(
        //     Polynomial::from_vec(&vec![0., 2.]).solve_roots(),
        //     Roots::One(Rational::from_i32(2)),
        // );
        // assert_eq!(
        //     Polynomial::from_vec(&vec![0., 1., 0.5]).solve_roots(),
        //     Roots::Two(Rational::from_i32(-2), Rational::from_i32(0)),
        // );
        // assert_eq!(
        //     Polynomial::from_vec(&vec![1., 0., 1.]).solve_roots(),
        //     Roots::Complex(Complex::new(0., -1.), Complex::new(0., 1.)),
        // );
    }
}
