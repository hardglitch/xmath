#[cfg(test)]
mod test_gprogression {
    use math::gprogression::{get_b_n, get_n, get_q, sum};

    #[test]
    fn test_gprogression_sum1_pos() {
        // sum = b1 * (q^n - 1) / (q - 1)
        let res = sum(3.0, 1.15, 5).unwrap();
        let q: f64 = 1.15;
        let res_test = 3.0 * (q.powf(5.0) - 1.0) / (q - 1.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_gprogression_sum1_neg() {
        let res = sum(3.0, 1.15, 5).unwrap();
        let q: f64 = 1.15;
        let res_test = 3.0 * (q.powf(5.0) - 2.0) / (q - 1.0);
        assert_ne!(res, res_test);
    }

    #[test]
    fn test_gprogression_sum2_neg() {
        let res = sum(3.0, 1.0, 5);
        assert!(res.is_err());
    }

    #[test]
    fn test_gprogression_get_n1_pos() {
        let res = get_n(3.0, 1.15, 20.0);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_gprogression_get_b_n1_pos() {
        // b_n = b_k * q^(n - k) , n > k
        let res = get_b_n(3.0, 1.15, 1, 6).unwrap();
        let q: f64 = 1.15;
        let res_test = 3.0 * (q.powf(6.0 - 1.0));
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_gprogression_get_b_n2_pos() {
        // b_n = b_k / q^(k - n) , n < k
        let res = get_b_n(3.0, 1.15, 6, 1).unwrap();
        let q: f64 = 1.15;
        let res_test = 3.0 / (q.powf(6.0 - 1.0));
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_gprogression_get_b_n2_neg() {
        let res = get_b_n(3.0, 0.0, 6, 1);
        assert!(res.is_err());
    }

    #[test]
    fn test_gprogression_get_q1_pos() {
        // q = (b_n / b_k)^(1 / (n - k)) , n > k
        let res = get_q(32.0, 1024.0, 2, 3).unwrap();
        let div: f64 = 1024.0 / 32.0;
        let res_test = div.powf(1.0 / (3.0 - 2.0));
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_gprogression_get_q2_pos() {
        // q = (b_k / b_n)^(1 / (k - n)) , n < k
        let res = get_q(1024.0, 32.0, 3, 2).unwrap();
        let div: f64 = 1024.0 / 32.0;
        let res_test = div.powf(1.0 / (3.0 - 2.0));
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_gprogression_get_q1_neg() {
        let res = get_q(32.0, 1024.0, 3, 3);
        assert!(res.is_err());
    }

    #[test]
    fn test_gprogression_get_q2_neg() {
        let res = get_q(32.0, 0.0, 3, 1);
        assert!(res.is_err());
    }

    #[test]
    fn test_gprogression_get_q3_neg() {
        let res = get_q(0.0, 1.0, 1, 3);
        assert!(res.is_err());
    }
}
