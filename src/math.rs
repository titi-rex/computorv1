pub fn gcd(a: &f32, b: &f32) -> f32 {
    let mut a = *a;
    let mut b = *b;
    let mut i = 0;

    while a % 2. == 0. && b % 2. == 0. {
        a /= 2.;
        b /= 2.;
        i += 1;
    }

    while a % 2. == 0. {
        a /= 2.;
    }

    while b % 2. == 0. {
        b /= 2.;
    }

    while a != b {
        if a > b {
            a -= b;
            while a % 2. == 0. {
                a /= 2.;
            }
        } else {
            b -= a;
            while b % 2. == 0. {
                b /= 2.;
            }
        }
    }

    pow(2., i) * a
}

pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

pub fn pow(mut x: f32, n: i32) -> f32 {
    if n == 0 {
        1.
    } else {
        for _i in 1..n {
            x *= x;
        }
        x
    }
}
