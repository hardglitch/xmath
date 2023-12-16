#[cfg(test)]
mod test_func_analysis {
    use xmath::func_analysis::{Expression, Point};
    use xmath::utils::AdvancedEQ;

    #[test]
    fn test_find_roots1_pos() {
        let mut y = Expression::new(
            |x: f64| x.powf(3.0) - 16.0 * x.powf(2.0) / 3.0 + 15.0 * x
        );
        y.find_roots().unwrap();
        let root = y.roots().unwrap()[0];
        assert_eq!(0.0, root);
    }

    #[test]
    fn test_find_roots2_none_pos() {
        let mut y = Expression::new(
            |x: f64| (x.powf(3.0) + 1.0) * (x - 1.0) / (x - 2.0).sqrt()
        );
        y.find_roots().unwrap();
        let root = y.roots();
        assert!(root.is_none());
    }

    #[test]
    fn test_find_extremums1_max_min_pos() {
        let mut y = Expression::new(
            |x: f64| 6.0 * x.powf(5.0) - 90.0 * x.powf(3.0) - 5.0
        );
        y.find_extremums(-5.0, 1.0).unwrap();

        let max = y.max().unwrap();
        assert_eq!(max, Point::new(-3.0, 967.0));
        assert_eq!(max.x(), -3.0);
        assert_eq!(max.y(), 967.0);

        let min = y.min().unwrap();
        assert_eq!(min, Point::new(-5.0, -7505.0));
        assert_eq!(min.x(), -5.0);
        assert_eq!(min.y(), -7505.0);
    }

    #[test]
    fn test_find_extremums2_extremums_pos() {
        let mut y = Expression::new(
            |x: f64| x.powf(3.0) - 16.0 * x.powf(2.0) / 3.0 + 15.0 * x
        );
        let res = y.find_extremums(-5.0, 1.0).unwrap().unwrap();
        let min = res.first().unwrap();
        let max = res.last().unwrap();
        assert!(min.x().is_equal(-5.0, 0.01));
        assert!(min.y().is_equal(-333.33, 0.01));
        assert!(max.x().is_equal(1.0, 0.01));
        assert!(max.y().is_equal(10.67, 0.01));
    }

    #[test]
    fn test_find_extremums3_extremums_pos() {
        let mut y = Expression::new(
            |x: f64| x.powf(2.0) + x
        );
        let res = y.find_extremums(-5.0, 1.0).unwrap().unwrap();
        let min = Point::new(-0.5, -0.25);
        let max = Point::new(-5.0, 20.0);
        assert!(min.eq(res.first().unwrap()));
        assert!(max.eq(res.last().unwrap()));
        assert_eq!(Some(vec![min, max]), y.extremums());
    }

    #[test]
    fn test_find_extremums4_extremums_pos() {
        let mut y = Expression::new(
            |x: f64| x.powf(2.0) + x
        );
        let res = y.find_extremums(-5.0, 1.0).unwrap().unwrap();
        let min = Point::new(-0.5, -0.25);
        let max = Point::new(-5.0, 20.0);
        assert!(min.eq(res.first().unwrap()));
        assert!(max.eq(res.last().unwrap()));
        assert_eq!(Some(vec![min, max]), y.extremums());
    }

    #[test]
    fn test_find_extremums5_none_pos() {
        let mut y = Expression::new(
            |x: f64| (x.powf(3.0) + 1.0) * (x - 1.0) / (x - 2.0).sqrt()
        );
        let res = y.find_extremums(-5.0, 1.0).unwrap();
        assert!(res.is_none());
        assert!(y.extremums().is_none());
    }

    #[test]
    fn test_output_result1_none_pos() {
        let y = Expression::new(
            |x: f64| (x.powf(3.0) + 1.0) * (x - 1.0) / (x - 2.0).sqrt()
        );
        assert!(y.extremums().is_none());
        assert!(y.min().is_none());
        assert!(y.max().is_none());
        assert!(y.roots().is_none());
    }
}