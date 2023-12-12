#[cfg(test)]
mod test_matrices {
    use xmath::matrices::Matrix;

    #[test]
    fn test_matrices1_pos() {
        let m = Matrix::new(3,3,vec![
            0.0, 2.0, -1.0,
            -2.0, -1.0, 2.0,
            3.0, -2.0, -1.0,
        ]).unwrap();
        let d = m.det();
        assert_eq!(d, 1.0);
    }

    #[test]
    fn test_matrices2_pos() {
        let m = Matrix::new(2,2,vec![
            131.0, 231.0,
            -130.0, -230.0,
        ]).unwrap();
        let d = m.det();
        assert_eq!(d, -100.0);
    }

    #[test]
    fn test_matrices3_pos() {
        let m = Matrix::new(2,2,vec![
            131.0, 231.0,
            0.0, -0.0,
        ]).unwrap();
        let d = m.det();
        assert_eq!(d, 0.0);
    }

    #[test]
    fn test_matrices4_pos() {
        let m = Matrix::new(2,3,vec![
            131.0, 231.0, 5.4,
            0.0, -0.0, 4.5,
        ]).unwrap();
        let d = m.det();
        assert_eq!(d, 0.0);
    }

    #[test]
    fn test_matrices5_pos() {
        let m = Matrix::new(5,5,vec![
            0.0, 5.0, 6.0, 7.0, 1.0,
            1.0, 4.0, 5.0, 1.0, 1.0,
            0.0, 3.0, 1.0, 2.0, 2.0,
            0.0, 1.0, 7.0, 8.0, 6.0,
            0.0, 1.0, 4.0, 4.0, 7.0,
        ]).unwrap();
        let d = m.det();
        assert_eq!(d, 156.0);
    }

    #[test]
    fn test_matrices6_pos() {
        let m1 = Matrix::new(1,3,vec![
            1.0, 2.0, 3.0,
        ]).unwrap();
        let m2 = Matrix::new(3,1,vec![
            1.0, 2.0, 3.0,
        ]).unwrap();
        let test_res = Matrix::new(1,1, vec![14.0]).unwrap();
        assert_eq!(m1 * m2, test_res);
    }

    #[test]
    fn test_matrices7_pos() {
        let m1 = Matrix::new(2,2,vec![
            1.0, 2.0,
            3.0, 4.0,
        ]).unwrap();
        let m2 = Matrix::new(2,2,vec![
            5.0, 6.0,
            7.0, 8.0,
        ]).unwrap();
        let test_res = Matrix::new(2,2, vec![
            19.0, 22.0,
            43.0, 50.0,
        ]).unwrap();
        assert_eq!(m1 * m2, test_res);
    }

    #[test]
    fn test_matrices8_pos() {
        let m1 = Matrix::new(2,2,vec![
            1.0, 2.0,
            3.0, 4.0,
        ]).unwrap();
        let m2 = Matrix::new(2,1,vec![
            1.0,
            2.0,
        ]).unwrap();
        let test_res = Matrix::new(2,1, vec![
            5.0,
            11.0,
        ]).unwrap();
        assert_eq!(m1 * m2, test_res);
    }

    #[test]
    fn test_matrices9_pos() {
        let m1 = Matrix::new(2,1,vec![
            1.0,
            2.0,
        ]).unwrap();
        let m2 = Matrix::new(2,2,vec![
            1.0, 2.0,
            3.0, 4.0,
        ]).unwrap();
        let test_res = Matrix::new(2,1, vec![
            5.0,
            11.0,
        ]).unwrap();
        assert_eq!(m1 * m2, test_res);
    }

    #[test]
    fn test_matrices10_pos() {
        let m1 = Matrix::new(3,3,vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]).unwrap();
        let m2 = Matrix::new(3,3,vec![
            1.0, 1.0, 2.0,
            3.0, -4.0, 5.0,
            6.0, 7.0, 8.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            2.0, 3.0, 5.0,
            7.0, 1.0, 11.0,
            13.0, 15.0, 17.0,
        ]).unwrap();
        assert_eq!(m1 + m2, test_res);
    }

    #[test]
    fn test_matrices11_pos() {
        let m1 = Matrix::new(3,3,vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]).unwrap();
        let m2 = Matrix::new(3,3,vec![
            1.0, 1.0, 2.0,
            3.0, -4.0, 5.0,
            6.0, 7.0, 8.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            0.0, 1.0, 1.0,
            1.0, 9.0, 1.0,
            1.0, 1.0, 1.0,
        ]).unwrap();
        assert_eq!(m1 - m2, test_res);
    }

    #[test]
    fn test_matrices12_pos() {
        let m1 = Matrix::new(3,3,vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            1.0, 2.0, 3.0,
        ]).unwrap();
        let m2 = Matrix::new(3,3,vec![
            1.0, 2.0, 3.0,
            1.0, 2.0, 3.0,
            5.0, 2.0, 1.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            18.0, 12.0, 12.0,
            39.0, 30.0, 33.0,
            18.0, 12.0, 12.0,
        ]).unwrap();
        assert_eq!(m1 * m2, test_res);
    }


    #[test]
    fn test_matrices13_pos() {
        let m = Matrix::new(3,3,vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            1.0, 2.0, 3.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            1.0, 4.0, 1.0,
            2.0, 5.0, 2.0,
            3.0, 6.0, 3.0,
        ]).unwrap();
        assert_eq!(m.transpose(), test_res);
    }

    #[test]
    fn test_matrices1_neg() {
        let m = Matrix::new(3,3,vec![
            131.0, 231.0, 5.4,
            0.0, -0.0, 4.5,
        ]);
        assert!(m.is_err());
    }

    #[test]
    fn test_matrices2_neg() {
        let m = Matrix::new(2,2,vec![
            131.0, 231.0, 5.4,
            0.0, -0.0, 4.5,
        ]);
        assert!(m.is_err());
    }

    #[test]
    fn test_matrices3_neg() {
        let m = Matrix::new(2,2,vec![]);
        assert!(m.is_err());
    }

    #[test]
    fn test_matrices4_neg() {
        let m = Matrix::new(2,0,vec![
            131.0, 231.0, 5.4,
            0.0, -0.0, 4.5,
        ]);
        assert!(m.is_err());
    }

    #[test]
    fn test_matrices5_neg() {
        let m = Matrix::new(0,2,vec![
            131.0, 231.0, 5.4,
            0.0, -0.0, 4.5,
        ]);
        assert!(m.is_err());
    }

    #[test]
    fn test_matrices6_neg() {
        let m1 = Matrix::new(2,1,vec![
            1.0,
            2.0,
        ]).unwrap();
        let m2 = Matrix::new(3,2,vec![
            1.0, 2.0,
            3.0, 4.0,
            1.0, 2.0,
        ]).unwrap();
        let test_res = Matrix::new(2,1, vec![
            5.0,
            11.0,
        ]).unwrap();
        assert_ne!(m1 * m2, test_res);
    }

    #[test]
    fn test_matrices7_neg() {
        let m1 = Matrix::new(3,2,vec![
            1.0, 2.0,
            3.0, 4.0,
            1.0, 2.0,
        ]).unwrap();
        let m2 = Matrix::new(2,1,vec![
            1.0,
            2.0,
        ]).unwrap();
        let test_res = Matrix::new(2,1, vec![
            5.0,
            11.0,
        ]).unwrap();
        assert_ne!(m1 * m2, test_res);
    }

    #[test]
    fn test_matrices8_neg() {
        let m1 = Matrix::new(3,3,vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]).unwrap();
        let m2 = Matrix::new(3,3,vec![
            1.0, 1.0, 2.0,
            3.0, -4.0, 5.0,
            6.0, 7.0, 8.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            1.0, 3.0, 5.0,
            7.0, 1.0, 11.0,
            13.0, 15.0, 17.0,
        ]).unwrap();
        assert_ne!(m1 + m2, test_res);
    }

    #[test]
    fn test_matrices9_neg() {
        let m1 = Matrix::new(3,3,vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]).unwrap();
        let m2 = Matrix::new(3,3,vec![
            1.0, 1.0, 2.0,
            3.0, -4.0, 5.0,
            6.0, 7.0, 8.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            1.0, 1.0, 1.0,
            1.0, 9.0, 1.0,
            1.0, 1.0, 1.0,
        ]).unwrap();
        assert_ne!(m1 - m2, test_res);
    }

    #[test]
    fn test_matrices10_neg() {
        let m = Matrix::new(3,3,vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            1.0, 2.0, 3.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            7.0, 4.0, 1.0,
            2.0, 5.0, 2.0,
            3.0, 6.0, 3.0,
        ]).unwrap();
        assert_ne!(m.transpose(), test_res);
    }
}