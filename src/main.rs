use crate::polynomial::{Polynomial, Roots};

mod polynomial;
mod complex_number;
mod rational;
mod math;
//      5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0
// red: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0

// A*x^0 + B*X^ 1 + C*x^2


fn main() {
    let mut eq = Polynomial::from([0.,1.,2.]);
    let right = Polynomial::from([0.,0.,1.]);

    eq -= right;
    println!("Reduced form: {}", eq);
    println!("Polynomial degree: {}", eq.degree());


    let solutions = Polynomial::solve_roots(&eq);
    match solutions {
        Roots::Zero => { println!("Zero solutions"); }
        Roots::Any => { println!("Any number is a solution"); }
        Roots::One(r) => {
            println!("Discriminant is zero");
            println!("One root: {}", r);
        }
        Roots::Two(r1, r2) => {
            println!("Discriminant is strictly positive");
            println!("Two root: {} and {}", r1, r2);
        }
        Roots::Complex(r1, r2) => {
            println!("Discriminant is strictly negative");
            println!("Two complex roots: {}, {}", r1, r2);
        }
    }




}
