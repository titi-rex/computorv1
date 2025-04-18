use std::fmt::{Display};
use std::ops::{Sub, SubAssign};
use crate::rational::Rational;

pub enum Roots<T> {
    Zero,
    Any,
    One(T),
    Two(T, T),
    Complex(T, T),
}

#[derive(Debug,Copy,Clone)]
pub struct Polynomial {
    a: f32,
    b: f32,
    c: f32,
}

impl Polynomial {
    pub fn new() -> Polynomial {
        Self {
            a: 0.,
            b: 0.,
            c: 0.,
        }
    }
    pub fn from(ar: [f32; 3]) -> Polynomial {
        Self {
            a: ar[0],
            b: ar[1],
            c: ar[2],
        }
    }
    pub fn degree(&self) -> i32 {
        if self.a != 0. {
            2
        } else if self.b != 0. {
            1
        } else if self.c != 0. {
            0
        } else {
            -1
        }
    }

    pub fn discriminant(p : &Polynomial) -> f32 {
        p.b * p.b - 4.0 * p.a * p.c
    }


    pub fn solve_roots(p : &Polynomial) -> Roots<Rational> {
        let degree = p.degree();

        match degree {
            0 => Roots::Any,
            1 => Self::solve_affine(p),
            2 => Self::solve_quadratic(p),
            _ => Roots::Zero,
        }
}

    fn solve_affine(p : &Polynomial) -> Roots<Rational> {
        // bx+c = 0  -> x = -c/b
        if p.b == 0. {
            Roots::Zero
        } else {
            Roots::One(Rational::new(-p.c, p.b))
        }
    }



    fn solve_quadratic(p : &Polynomial) -> Roots<Rational> {

        match Polynomial::discriminant(p) {
            d if d > 0.0 => {
                // r = (-b +/- sqrt(D)) / 2a
                let r1 = Rational::new(-p.b - d.sqrt(), 2.*p.a);
                let r2 = Rational::new(-p.b + d.sqrt(), 2.*p.a);
                Roots::Two(r1, r2)
            },
            d if d < 0.0  => {
                // r = (-b +/- i *sqrt(-D)) / 2a

                Roots::Zero
                // Roots::TwoComplex(base - dbase, base + dbase)
            },
            _  => {
                // r = -b/2a
                let r = Rational::new(-p.b, 2.*p.a);
                Roots::One(r)
            },
        }
    }

}



impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s1 = String::new();
        let mut s2 = String::new();
        let mut s3 = String::new();

        if self.c != 0. {
            s1 = std::fmt::format(format_args!("{}", self.c));
        }
        if self.b != 0. {
            s2 = std::fmt::format(format_args!(" + {} * X", self.b));
        }
        if self.a != 0. {
            s3 = std::fmt::format(format_args!(" + {} * X²", self.a));
        }
        if self.a == 0. && self.b == 0. && self.c == 0. {
            s1 = "0".to_string();
        }
        write!(f, "{}{}{} = 0", s1, s2, s3)
    }
}

impl Sub for Polynomial{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c,
        }
    }
}

impl SubAssign for Polynomial{
    fn sub_assign(&mut self, rhs: Self) {
        self.a -= rhs.a;
        self.b -= rhs.b;
        self.c -= rhs.c;
    }
}