#[cfg(test)]
mod test_theory_of_probability {
    use xmath::top::{bernoulli, binom};
    #[test]
    fn test_binominal_coefficient1_pos() {
        // C_n^m = n! / m! * (n - m)!
        let res = binom(4, 5).unwrap();
        let res_test = (2.0 * 3.0 * 4.0 * 5.0) / (2.0 * 3.0 * 4.0) * (5.0 - 4.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_binominal_coefficient1_neg() {
        let res = binom(6, 5);
        assert!(res.is_err());
    }

    #[test]
    fn test_binominal_coefficient2_neg() {
        let res = binom(5, 5);
        assert!(res.is_err());
    }

    #[test]
    fn test_binominal_coefficient3_neg() {
        let res = binom(0, 5);
        assert!(res.is_err());
    }

    #[test]
    fn test_bernoulli1_pos() {
        // P_n^m = C_n^m * p^m * q^(n-m)
        let res = bernoulli(7, 15, 0.5, 0.5).unwrap();
        let res_test = binom(7, 15).unwrap() * 0.5_f64.powf(7.0) * 0.5_f64.powf(15.0-7.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_bernoulli1_neg() {
        let res = bernoulli(15, 7, 0.5, 0.5);
        assert!(res.is_err());
    }
}
