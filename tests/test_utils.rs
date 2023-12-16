#[cfg(test)]
mod test_utils {
    use xmath::utils::AdvancedEQ;

    #[test]
    fn test_is_equal1_pos() {
        let res = 0.001.is_equal(0.001, 0.001);
        assert!(res);
    }

    #[test]
    fn test_is_equal2_pos() {
        let res = 0.0012.is_equal(0.0013, 0.001);
        assert!(res);
    }

    #[test]
    fn test_is_equal3_pos() {
        let res = 0.0012.is_equal(0.0013, 0.0001);
        assert!(!res);
    }

    #[test]
    fn test_is_equal4_pos() {
        let res = 0.000001.is_equal(0.001, 0.0001);
        assert!(!res);
    }

    #[test]
    fn test_is_equal5_pos() {
        let res = 0.000001.is_equal(0.001, 0.001);
        assert!(res);
    }

    #[test]
    fn test_is_equal6_pos() {
        let res = 0.00167.is_equal(0.0, 0.01);
        assert!(res);
    }

    #[test]
    fn test_is_equal7_pos() {
        let res = 0.001.is_equal(0.005, 0.001);
        assert!(!res);
    }

    #[test]
    fn test_is_equal8_pos() {
        let res = -1.1.is_equal(-5.3, -0.001);
        assert!(!res);
    }
}
