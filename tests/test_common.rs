#[cfg(test)]
mod test_common {
    use xmath::common::{factorial, sigma, simple_sigma};
    use xmath::utils::AdvancedEQ;

    #[test]
    fn test_factorial1_pos() {
        // n! = 2 * 3 * ... * n
        let res = factorial(5);
        let res_test = 2.0 * 3.0 * 4.0 * 5.0;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_factorial2_pos() {
        let res = factorial(0);
        assert_eq!(res, 0.0);
    }

    #[test]
    fn test_factorial3_pos() {
        let res = factorial(1);
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_sigma1_pos() {
        let res = sigma(1.0, 1.5, 0.1, |x| x + 1.0).unwrap();
        assert!(res.is_equal(13.5, 0.01));
    }

    #[test]
    fn test_sigma2_pos() {
        let res = sigma(1.5, 1.0, -0.1, |x| x + 1.0).unwrap();
        assert!(res.is_equal(13.5, 0.01));
    }

    #[test]
    fn test_sigma3_pos() {
        let res = sigma(1.0, 2.5, 1.0, |x| x + 1.0).unwrap();
        assert!(res.is_equal(5.0, 0.01));
    }

    #[test]
    fn test_sigma1_neg() {
        let res = sigma(1.5, 1.5, -0.1, |x| x + 1.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_sigma2_neg() {
        let res = sigma(1.0, 1.5, -0.1, |x| x + 1.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_sigma3_neg() {
        let res = sigma(1.5, 1.0, 0.1, |x| x + 1.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_sigma4_neg() {
        let res = sigma(1.0, 1.5, -0.0, |x| x + 1.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_sigma5_neg() {
        let res = sigma(1.0, 1.5, 2.0, |x| x + 1.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_sigma6_neg() {
        let res = sigma(1.5, 1.0, -2.0, |x| x + 1.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_simple_sigma1_pos() {
        let res = simple_sigma(5);
        let test_res = 1 + 2 + 3 + 4 + 5;
        assert_eq!(res, test_res);
    }

    #[test]
    fn test_simple_sigma2_pos() {
        let res = simple_sigma(0);
        assert_eq!(res, 0);
    }
}
