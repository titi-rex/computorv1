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

pub fn pow(x: i32, n: i32) -> i32 {
    x.pow(n as u32)
}

pub fn number_len(n: u64) -> usize {
    n.checked_ilog10().unwrap_or(0) as usize + 1
}

#[cfg(test)]
mod test {
    use super::*;
    use gcd::binary_u64;

    #[test]
    fn sqrt_test() {
        assert_eq!(9f32.sqrt(), sqrt(9f32));
        assert_eq!(514f32.sqrt(), sqrt(514f32));
        assert_eq!(0f32.sqrt(), sqrt(0f32));
    }

    #[test]
    fn number_len_test() {
        assert_eq!(number_len(94_521), 5);
        assert_eq!(number_len(0), 1);
        assert_eq!(number_len(251), 3);
        assert_eq!(number_len(1_000_000), 7);
    }

    #[test]
    fn pow_test() {
        assert_eq!(pow(10, 2), 10i32.pow(2));
        assert_eq!(pow(0, 2), 0i32.pow(2));
        assert_eq!(pow(2, 10), 2i32.pow(10));
    }

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(9, 2), binary_u64(9, 2));
    }
}
