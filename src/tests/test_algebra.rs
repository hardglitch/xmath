#[cfg(test)]
mod test_algebra {
    use crate::algebra::quadratic_equation;
    use crate::utils::AdvancedEQ;

    #[test]
    fn test_quadratic_equation1_pos() {
        let res = quadratic_equation(5.0, -63.0, -18.0).unwrap().unwrap();
        assert!(res.first().unwrap().is_equal(12.88, 0.01));
        assert!(res.last().unwrap().is_equal(-0.28, 0.01));
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
}
