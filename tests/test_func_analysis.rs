#[cfg(test)]
mod test_func_analysis {
    use math::func_analysis::{Expression, Point};
    use std::collections::HashMap;
    use math::utils::is_equal;

    #[test]
    fn test_find_roots1_pos() {
        let mut y = Expression::new(
            |x: f64| x.powf(3.0) - 16.0 * x.powf(2.0) / 3.0 + 15.0 * x
        );
        y.find_roots().unwrap();
        let root = y.roots()[0].unwrap();
        assert_eq!(0.0, root);
    }

    #[test]
    fn test_find_roots2_none_pos() {
        let mut y = Expression::new(
            |x: f64| (x.powf(3.0) + 1.0) * (x - 1.0) / (x - 2.0).sqrt()
        );
        y.find_roots().unwrap();
        let root = y.roots()[0];
        assert!(root.is_none());
    }

    #[test]
    fn test_find_extremums1_max_min_pos() {
        let mut y = Expression::new(
            |x: f64| 6.0 * x.powf(5.0) - 90.0 * x.powf(3.0) - 5.0
        );
        y.find_extremums(-5.0, 1.0).unwrap();

        let max = y.max().unwrap();
        assert_eq!(max, &Point{x:-3.0, y:967.0});
        assert_eq!(max.x, -3.0);
        assert_eq!(max.y, 967.0);

        let min = y.min().unwrap();
        assert_eq!(min, &Point{x:-5.0, y:-7505.0});
        assert_eq!(min.x, -5.0);
        assert_eq!(min.y, -7505.0);
    }

    #[test]
    fn test_find_extremums2_extremums_pos() {
        let mut y = Expression::new(
            |x: f64| x.powf(3.0) - 16.0 * x.powf(2.0) / 3.0 + 15.0 * x
        );
        let res = y.find_extremums(-5.0, 1.0).unwrap().unwrap();
        let min = res.get("min").unwrap();
        let max = res.get("max").unwrap();
        assert!(is_equal(&min.x, &-5.0, 0.01));
        assert!(is_equal(&min.y, &-333.33, 0.01));
        assert!(is_equal(&max.x, &1.0, 0.01));
        assert!(is_equal(&max.y, &10.67, 0.01));
    }

    #[test]
    fn test_find_extremums3_extremums_pos() {
        let mut y = Expression::new(
            |x: f64| x.powf(2.0) + x
        );
        let res = y.find_extremums(-5.0, 1.0).unwrap();
        let mut test_data = HashMap::<String, Point>::new();
        test_data.insert("min".to_owned(), Point{ x: -0.5, y: -0.25 });
        test_data.insert("max".to_owned(), Point{ x: -5.0, y: 20.0 });
        assert_eq!(Some(&test_data), res);
        assert_eq!(Some(&test_data), y.extremums());
    }

    #[test]
    fn test_find_extremums4_extremums_pos() {
        let mut y = Expression::new(
            |x: f64| x.powf(2.0) + x
        );
        let res = y.find_extremums(1.0, -5.0).unwrap();
        let mut test_data = HashMap::<String, Point>::new();
        test_data.insert("min".to_owned(), Point{ x: -0.5, y: -0.25 });
        test_data.insert("max".to_owned(), Point{ x: -5.0, y: 20.0 });
        assert_eq!(Some(&test_data), res);
        assert_eq!(Some(&test_data), y.extremums());
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
}