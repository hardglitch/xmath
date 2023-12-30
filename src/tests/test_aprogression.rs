#[cfg(test)]
mod test_aprogression {
    use crate::aprogression::{get_a_n, get_d, sum_by_d, sum_by_elems, get_iters};

    #[test]
    fn test_aprogression_sum_by_d1_pos() {
        let res = sum_by_d(-2.0, 2, 8, 1.0).unwrap();
        assert_eq!(res, 7.0);
    }

    #[test]
    fn test_aprogression_sum_by_d2_pos() {
        let res = sum_by_d(-2.0, 8, 2, 1.0).unwrap();
        assert_eq!(res, 7.0);
    }

    #[test]
    fn test_aprogression_sum_by_d1_neg() {
        let res = sum_by_d(-2.0, 8, 8, 1.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_aprogression_sum_by_elem1_pos() {
        let res = sum_by_elems(-2.0, 8, -8.0, 2).unwrap();
        assert_eq!(res, 7.0);
    }

    #[test]
    fn test_aprogression_sum_by_elem2_pos() {
        let res = sum_by_elems(-8.0, 2, -2.0, 8).unwrap();
        assert_eq!(res, 7.0);
    }

    #[test]
    fn test_aprogression_sum_by_elem1_neg() {
        let res = sum_by_elems(-2.0, 8, -8.0, 8);
        assert!(res.is_err());
    }

    #[test]
    fn test_aprogression_get_d1_pos() {
        let res = get_d(-2.0, 2, 4.0, 8).unwrap();
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_aprogression_get_d1_neg() {
        let res = get_d(-2.0, 2, 4.0, 2);
        assert!(res.is_err());
    }

    #[test]
    fn test_aprogression_get_iters1_pos() {
        let res = get_iters(60.0, 17.0, 1000.0).unwrap();
        assert_eq!(res, 9);
    }

    #[test]
    fn test_aprogression_get_iters1_neg() {
        let res = get_iters(1.0, 0.0, 1000.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_aprogression_get_iters2_neg() {
        let res = get_iters(1.0, 1.0, 0.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_aprogression_get_iters3_neg() {
        let res = get_iters(-1.0, 1.0, 1000.0);
        assert!(res.is_err());
    }

    #[test]
    fn test_aprogression_get_a_n1_pos() {
        let res = get_a_n(3.0, 2, 6, 2.0).unwrap();
        assert_eq!(res, 11.0);
    }

    #[test]
    fn test_aprogression_get_a_n2_pos() {
        // a_n = a_k - (k - n) * d
        let res = get_a_n(3.0, 6, 2, 2.0).unwrap();
        assert_eq!(res, -5.0);
    }
}
