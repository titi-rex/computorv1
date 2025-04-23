use computorv1::polynomial::{Polynomial, Roots};
//      5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0
// red: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0

// A*x^0 + B*X^ 1 + C*x^2

fn main() {
    let input_raw = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("Please provide an expression");
            return;
        }
    };

    println!("input: {}", input_raw);
    let eqs = input_raw.split("=").collect::<Vec<&str>>();
    if eqs.len() != 2 {
        println!("expression can only have one =");
        return;
    }
    let p1 = Polynomial::from_str(eqs[0]).expect("Bad expression");
    let p2 = Polynomial::from_str(eqs[1]).expect("Bad expression");
    println!("p1: {}", p1);
    println!("p2: {:?}", p2);

    let eq = p1 - p2;

    println!("Reduced form: {}", eq);
    println!("Polynomial degree: {}", eq.degree());

    let solutions = Polynomial::solve_roots(&eq);
    match solutions {
        Roots::Zero => {
            println!("Zero solutions");
        }
        Roots::Any => {
            println!("Any number is a solution");
        }
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
