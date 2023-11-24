#[cfg(test)]
mod test_algebra {
    use math::algebra::quadratic_equation;

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
}
