use computorv1::polynomial::Polynomial;
use computorv1::root::Roots;

fn main() {
    let input_raw = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            return println!("Please provide an expression");
        }
    };

    let eqs = input_raw.split("=").collect::<Vec<&str>>();
    if eqs.len() != 2 {
        return println!("expression can only have one =");
    }

    let eq = match (Polynomial::from_str(eqs[0]), Polynomial::from_str(eqs[1])) {
        (Ok(p1), Ok(p2)) => p1 - p2,
        _ => return println!("Invalid expression"),
    };

    println!("Reduced form: {} = 0", eq);
    let degree = eq.degree();
    if degree > 0 {
        println!("Polynomial degree: {}", degree);
    }

    if degree >= 3 {
        return println!("The polynomial degree is strictly greater than 2, I can't solve.");
    }

    match eq.solve_roots() {
        Roots::Unsolvable => println!("{}", Roots::Unsolvable),
        Roots::Zero => println!("{}", Roots::Zero),
        Roots::Any => println!("{}", Roots::Any),
        Roots::One(r) => println!("Discriminant is zero\n{}", Roots::One(r)),
        Roots::Two(r1, r2) => println!("Discriminant is strictly positive\n{}", Roots::Two(r1, r2)),
        Roots::Complex(r) => println!("Discriminant is strictly negative\n{}", Roots::Complex(r)),
    }
}


// "-4 * x^0 + 6 * x^1 + 2 * x^2 =  -4 * x^0"