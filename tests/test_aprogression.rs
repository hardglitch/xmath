#[cfg(test)]
mod test_aprogression {
    use math::aprogression::{get_a_n, get_n, sum};

    #[test]
    fn test_aprogression_sum1_pos() {
        // sum = a1 * n + d * sigma(n - 1)
        let res = sum(2.0, 1.5, 4);
        let res_test = 2.0 * 4.0 + 1.5 * (3.0 + 2.0 + 1.0);
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_aprogression_sum1_neg() {
        let res = sum(2.0, 1.2, 4);
        let res_test = 2.0 * 4.0 + 1.5 * (3.0 + 2.0 + 1.0);
        assert_ne!(res, res_test);
    }

    #[test]
    fn test_aprogression_get_n1_pos() {
        let res = get_n(60.0, 17.0, 1000.0);
        assert_eq!(res, 9);
    }

    #[test]
    fn test_aprogression_get_a_n1_pos() {
        // a_n = a_k + (n - k) * d , n => k
        let res = get_a_n(3.0, 2.0, 2, 6);
        let res_test = 3.0 + (6.0 - 2.0) * 2.0;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_aprogression_get_a_n2_pos() {
        // a_n = a_k - (k - n) * d , n <= k
        let res = get_a_n(3.0, 2.0, 6, 2);
        let res_test = 3.0 - (6.0 - 2.0) * 2.0;
        assert_eq!(res, res_test);
    }
}
