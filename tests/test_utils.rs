#[cfg(test)]
mod test_utils {
    use xmath::utils::is_equal;

    #[test]
    fn test_is_equal1_pos() {
        let res = is_equal(&0.001, &0.001, 0.001);
        assert!(res);
    }

    #[test]
    fn test_is_equal2_pos() {
        let res = is_equal(&0.0012, &0.0013, 0.001);
        assert!(res);
    }

    #[test]
    fn test_is_equal3_pos() {
        let res = is_equal(&0.0012, &0.0013, 0.0001);
        assert!(!res);
    }

    #[test]
    fn test_is_equal4_pos() {
        let res = is_equal(&0.000001, &0.001, 0.0001);
        assert!(!res);
    }

    #[test]
    fn test_is_equal5_pos() {
        let res = is_equal(&0.000001, &0.001, 0.001);
        assert!(res);
    }

    #[test]
    fn test_is_equal6_pos() {
        let res = is_equal(&0.00167, &0.0, 0.01);
        assert!(res);
    }
}
