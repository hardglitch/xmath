#[cfg(test)]
mod all_tests {
    use crate::math::*;

    #[test]
    fn test_factorial1_pos() {
        // n! = 2 * 3 * ... * n
        let res = factorial(5);
        let res_test: u128 = 2 * 3 * 4 * 5;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_factorial2_pos() {
        let res = factorial(0);
        let res_test: u128 = 0;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_factorial3_pos() {
        let res = factorial(1);
        let res_test: u128 = 1;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_binominal_coefficient1_pos() {
        // C_n^m = n! / m! * (n - m)!
        let res = binominal_coefficient(4, 5).unwrap();
        let res_test: f64 = (2.0 * 3.0 * 4.0 * 5.0) / (2.0 * 3.0 * 4.0) * (5.0 - 4.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_binominal_coefficient1_neg() {
        let res = binominal_coefficient(6, 5);
        assert!(res.is_err());
    }

    #[test]
    fn test_bernoulli1_pos() {
        // P_n^m = C_n^m * p^m * q^(n-m)
        let q: f64 = 0.5;
        let p: f64 = 0.5;
        let res = bernoulli(7, 15, p, q).unwrap();
        let res_test: f64 = binominal_coefficient(7, 15).unwrap() * p.powf(7.0) * q.powf(15.0-7.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_bernoulli1_neg() {
        let res = bernoulli(15, 7, 0.5, 0.5);
        assert!(res.is_err());
    }

    #[test]
    fn test_quadratic_equation1_pos() {
        // +-ax^2 +-bx +-с = 0
        let a = -1.0;
        let b: f64 = 6.0;
        let c = -2.0;
        let d = b.powf(2.0) - 4.0 * a * c;
        let x1 = (-b + d.sqrt()) / 2.0 * a;
        let x2 = (-b - d.sqrt()) / 2.0 * a;
        let res_test = Some([x1, x2]);

        let res = quadratic_equation(a, b, c).unwrap();
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_quadratic_equation2_pos() {
        let res = quadratic_equation(-7.0, 6.0, -2.0).unwrap();
        assert_eq!(res, None);
    }

    #[test]
    fn test_quadratic_equation3_pos() {
        let res = quadratic_equation(-7.0, 0.0, -2.0).unwrap();
        assert_eq!(res, None);
    }

    #[test]
    fn test_quadratic_equation4_pos() {
        let res = quadratic_equation(-7.0, 0.0, 0.0).unwrap();
        assert_eq!(res, Some([0.0; 2]));
    }

    #[test]
    fn test_quadratic_equation1_neg() {
        let res = quadratic_equation(0.0, 6.0, -2.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_sigma1_pos() {
        // sigma(n) = 1 + 2 + 3 + ... + n
        let res = sigma(5);
        let res_test = 1 + 2 + 3 + 4 + 5;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_sigma1_neg() {
        let res = sigma(5);
        assert_ne!(res, 2);
    }

        #[test]
    fn test_aprogression_sum1_pos() {
        // sum = a1 * n + d * sigma(n - 1)
        let res = AProgression::sum(2.0, 1.5, 4);
        let res_test = 2.0 * 4.0 + 1.5 * (3.0 + 2.0 + 1.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_aprogression_sum1_neg() {
        let res = AProgression::sum(2.0, 1.2, 4);
        let res_test = 2.0 * 4.0 + 1.5 * (3.0 + 2.0 + 1.0);
        assert_ne!(res, res_test);
    }

    #[test]
    fn test_aprogression_get_n1_pos() {
        let res = AProgression::get_n(60.0, 17.0, 1000.0);
        assert_eq!(res, 9);
    }

    #[test]
    fn test_aprogression_get_a_n1_pos() {
        // a_n = a_k + (n - k) * d , n => k
        let res = AProgression::get_a_n(3.0, 2.0, 2, 6);
        let res_test = 3.0 + (6 - 2) as f64 * 2.0;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_aprogression_get_a_n2_pos() {
        // a_n = a_k - (k - n) * d , n <= k
        let res = AProgression::get_a_n(3.0, 2.0, 6, 2);
        let res_test = 3.0 - (6 - 2) as f64 * 2.0;
        assert_eq!(res, res_test);
    }


    #[test]
    fn test_gprogression_sum1_pos() {
        // sum = b1 * (q^n - 1) / (q - 1)
        let res = GProgression::sum(3.0, 1.15, 5).unwrap();
        let q: f64 = 1.15;
        let res_test = 3.0 * (q.powf(5.0) - 1.0) / (q - 1.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_gprogression_sum1_neg() {
        let res = GProgression::sum(3.0, 1.15, 5).unwrap();
        let q: f64 = 1.15;
        let res_test = 3.0 * (q.powf(5.0) - 2.0) / (q - 1.0);
        assert_ne!(res, res_test);
    }

    #[test]
    fn test_gprogression_sum2_neg() {
        let res = GProgression::sum(3.0, 1.0, 5);
        assert!(res.is_err());
    }

    #[test]
    fn test_gprogression_get_n1_pos() {
        let res = GProgression::get_n(3.0, 1.15, 20.0);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_gprogression_get_b_n1_pos() {
        // b_n = b_k * q^(n - k) , n => k
        let res = GProgression::get_b_n(3.0, 1.15, 1, 6).unwrap();
        let q: f64 = 1.15;
        let res_test = 3.0 * (q.powf(6.0 - 1.0));
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_gprogression_get_b_n2_pos() {
        // b_n = b_k / q^(k - n) , n <= k
        let res = GProgression::get_b_n(3.0, 1.15, 6, 1).unwrap();
        let q: f64 = 1.15;
        let res_test = 3.0 / (q.powf(6.0 - 1.0));
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_gprogression_get_b_n2_neg() {
        let res = GProgression::get_b_n(3.0, 0.0, 6, 1);
        assert!(res.is_err());
    }
}
