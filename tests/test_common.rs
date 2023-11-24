#[cfg(test)]
mod test_common {
    use math::common::{factorial, sigma};

    #[test]
    fn test_factorial1_pos() {
        // n! = 2 * 3 * ... * n
        let res = factorial(&5);
        let res_test: u128 = 2 * 3 * 4 * 5;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_factorial2_pos() {
        let res = factorial(&0);
        let res_test: u128 = 0;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_factorial3_pos() {
        let res = factorial(&1);
        let res_test: u128 = 1;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_sigma1_pos() {
        // sigma(n) = 1 + 2 + 3 + ... + n
        let res = sigma(&5);
        let res_test = 1 + 2 + 3 + 4 + 5;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_sigma1_neg() {
        let res = sigma(&5);
        assert_ne!(res, 2);
    }
}
