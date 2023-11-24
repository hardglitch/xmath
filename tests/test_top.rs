#[cfg(test)]
mod test_theory_of_probability {
    use math::top::{bernoulli, binominal_coefficient};
    #[test]
    fn test_binominal_coefficient1_pos() {
        // C_n^m = n! / m! * (n - m)!
        let res = binominal_coefficient(&4, &5).unwrap();
        let res_test: f64 = (2.0 * 3.0 * 4.0 * 5.0) / (2.0 * 3.0 * 4.0) * (5.0 - 4.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_binominal_coefficient1_neg() {
        let res = binominal_coefficient(&6, &5);
        assert!(res.is_err());
    }

    #[test]
    fn test_bernoulli1_pos() {
        // P_n^m = C_n^m * p^m * q^(n-m)
        let q: f64 = 0.5;
        let p: f64 = 0.5;
        let res = bernoulli(&7, &15, &p, &q).unwrap();
        let res_test: f64 = binominal_coefficient(&7, &15).unwrap() * p.powf(7.0) * q.powf(15.0-7.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_bernoulli1_neg() {
        let res = bernoulli(&15, &7, &0.5, &0.5);
        assert!(res.is_err());
    }
}
