pub fn sqrt_iter(x: f32, s: f32) -> f32 {
    0.5 * (x + s / x)
}

pub fn sqrt(x: f32) -> f32 {
    let mut r = 1.;
    for _i in 1..30 {
        r = sqrt_iter(r, x);
    }
    r
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sqrt_test() {
        assert_eq!(9f32.sqrt(), sqrt(9f32));
        assert_eq!(514f32.sqrt(), sqrt(514f32));
        assert_eq!(1f32.sqrt(), sqrt(1f32));
    }
}
