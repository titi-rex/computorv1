

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    let mut i: u32 = 0;

    while a % 2 == 0 && b % 2 == 0 {
        a /= 2;
        b /= 2;
        i += 1;
    }

    while a % 2 == 0 {
        a /= 2;
    }

    while b % 2 == 0 {
        b /= 2;
    }

    while a != b {
        if a > b {
            a -= b;
            while a % 2 == 0 {
                a /= 2;
            }
        } else {
            b -= a;
            while b % 2 == 0 {
                b /= 2;
            }
        }
    }

    2u64.pow(i as u32) * a
    // pow(2, i) as u64 * a
}

pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

pub fn pow(mut x: i32, n: i32) -> u32 {
    if n <= 0 {
        1
    } else {
        for _i in 1..n {
            x *= x;
        }
        x as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sqrt_test() {
        let n = 9f32;
        assert_eq!(n.sqrt(), sqrt(n));
    }
}
