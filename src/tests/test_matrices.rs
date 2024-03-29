#[cfg(test)]
mod test_matrices {
    use crate::matrices::Matrix;

    #[test]
    fn test_matrices_det1_pos() {
        let m = Matrix::new(3,3,vec![
            0.0, 2.0, -1.0,
            -2.0, -1.0, 2.0,
            3.0, -2.0, -1.0,
        ]).unwrap();
        let d = m.det();
        assert_eq!(d, 1.0);
    }

    #[test]
    fn test_matrices_det2_pos() {
        let m = Matrix::new(2,2,vec![
            131.0, 231.0,
            -130.0, -230.0,
        ]).unwrap();
        let d = m.det();
        assert_eq!(d, -100.0);
    }

    #[test]
    fn test_matrices_det3_pos() {
        let m = Matrix::new(2,2,vec![
            131.0, 231.0,
            0.0, -0.0,
        ]).unwrap();
        let d = m.det();
        assert_eq!(d, 0.0);
    }

    #[test]
    fn test_matrices_det4_pos() {
        let m = Matrix::new(2,3,vec![
            131.0, 231.0, 5.4,
            0.0, -0.0, 4.5,
        ]).unwrap();
        let d = m.det();
        assert_eq!(d, 0.0);
    }

    #[test]
    fn test_matrices_det5_pos() {
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
    fn test_matrices_mul1_pos() {
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
    fn test_matrices_mul2_pos() {
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
    fn test_matrices_mul3_pos() {
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
    fn test_matrices_mul4_pos() {
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
    fn test_matrices_mul5_pos() {
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
    fn test_matrices_mul_num1_pos() {
        let m = Matrix::new(3,3,vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            1.0, 2.0, 3.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            2.0, 4.0, 6.0,
            8.0, 10.0, 12.0,
            2.0, 4.0, 6.0,
        ]).unwrap();
        assert_eq!(m.mul_num(2.0), test_res);
    }

    #[test]
    fn test_matrices_add1_pos() {
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
    fn test_matrices_sub1_pos() {
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
    fn test_matrices_trans1_pos() {
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
    fn test_matrices_trans_2_pos() {
        let m = Matrix::new(2,3,vec![
            1.0, 1.0, 2.0,
            3.0, -4.0, 5.0,
        ]).unwrap();
        let test_res = Matrix::new(3,2,vec![
            1.0, 3.0,
            1.0, -4.0,
            2.0, 5.0,
        ]).unwrap();
        assert_eq!(m.transpose(), test_res);
    }

    #[test]
    fn test_matrices_inverse1_pos() {
        let m = Matrix::new(3,3,vec![
            2.0, 1.0, 3.0,
            4.0, 1.0, 5.0,
            4.0, 5.0, 2.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            -23.0/14.0, 13.0/14.0, 1.0/7.0,
            6.0/7.0, -4.0/7.0, 1.0/7.0,
            8.0/7.0, -3.0/7.0, -1.0/7.0,
        ]).unwrap();
        assert_eq!(m.inverse().unwrap(), test_res);
    }

    #[test]
    fn test_matrices_cfm1_pos() {
        let m = Matrix::new(3,3,vec![
            2.0, 1.0, 3.0,
            4.0, 1.0, 5.0,
            4.0, 5.0, 2.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            -23.0, 12.0, 16.0,
            13.0, -8.0, -6.0,
            2.0, 2.0, -2.0,
        ]).unwrap();
        assert_eq!(m.cofactor_matrix().unwrap(), test_res);
    }

    #[test]
    fn test_matrices_cfm2_pos() {
        let m = Matrix::new(2, 2, vec![
            2.0, 1.0,
            3.0, 2.0,
        ]).unwrap();
        let test_res = Matrix::new(2,2,vec![
            2.0, -3.0,
            -1.0, 2.0,
        ]).unwrap();
        assert_eq!(m.cofactor_matrix().unwrap(), test_res);
    }

    #[test]
    fn test_matrices_pow1_pos() {
        let m = Matrix::new(3,3,vec![
            1.0, 1.0, 2.0,
            3.0, -4.0, 5.0,
            6.0, 7.0, 8.0,
        ]).unwrap();
        assert_eq!(m.pow(3).unwrap(), m.clone() * m.clone() * m);
    }

    #[test]
    fn test_matrices_pow2_pos() {
        let m = Matrix::new(2,2,vec![
            1.0, 1.0,
            1.0, 0.0,
        ]).unwrap();
        assert_eq!(m.pow(8).unwrap(), m.clone() * m.clone() * m.clone() * m.clone() * m.clone() * m.clone() * m.clone() * m.clone());
    }

    #[test]
    fn test_matrices_pow3_pos() {
        let m = Matrix::new(2,2,vec![
            1., 1.,
            1., 0.,
        ]).unwrap();
        let res = Matrix::new(2,2,vec![
            34., 21.,
            21., 13.,
        ]).unwrap();
        assert_eq!(m.pow(8).unwrap(), res);
    }

    #[test]
    fn test_matrices_slae1_pos() {
        let m = Matrix::new(3,3,vec![
            1.0, 4.0, 2.0,
            2.0, -6.0, -2.0,
            1.0, 5.0, 2.0,
        ]).unwrap();
        let d = &[1.0, 3.0, 2.0];
        let test_res = vec![2.0, 1.0, -2.5];
        assert_eq!(m.slae(d).unwrap().unwrap(), test_res);
    }

    #[test]
    fn test_matrices_slae2_pos() {
        let m = Matrix::new(2,3,vec![
            1.0, 4.0, 2.0,
            2.0, -6.0, -2.0,
        ]).unwrap();
        let d = &[1.0, 3.0, 2.0];
        assert!(m.slae(d).unwrap().is_none());
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
    fn test_matrices_mul1_neg() {
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
    fn test_matrices_mul2_neg() {
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
    fn test_matrices_add1_neg() {
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
    fn test_matrices_sub1_neg() {
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
    fn test_matrices_trans1_neg() {
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

    #[test]
    fn test_matrices_inverse1_neg() {
        let m = Matrix::new(3,3,vec![
            2.0, 1.0, 3.0,
            4.0, 1.0, 5.0,
            4.0, 5.0, 2.0,
        ]).unwrap();
        let test_res = Matrix::new(3,3,vec![
            -23.0/14.0, 13.0/1.0, 1.0/7.0,
            6.0/7.0, -4.0/7.0, 1.0/7.0,
            8.0/7.0, -3.0/7.0, -1.0/7.0,
        ]).unwrap();
        assert_ne!(m.inverse().unwrap(), test_res);
    }

    #[test]
    fn test_matrices_inverse2_neg() {
        let m = Matrix::new(2,3,vec![
            2.0, 1.0, 3.0,
            4.0, 1.0, 5.0,
        ]).unwrap();
        assert!(m.inverse().is_none());
    }

    #[test]
    fn test_matrices_cfm1_neg() {
        let m = Matrix::new(2,3,vec![
            2.0, 1.0, 3.0,
            4.0, 1.0, 5.0,
        ]).unwrap();
        assert!(m.cofactor_matrix().is_none());
    }

    #[test]
    fn test_matrices_pow1_neg() {
        let m = Matrix::new(2,3,vec![
            1.0, 1.0, 2.0,
            3.0, -4.0, 5.0,
        ]).unwrap();
        assert!(m.pow(3).is_err());
    }

    #[test]
    fn test_matrices_slae1_neg() {
        let m = Matrix::new(3,3,vec![
            1.0, 1.0, 2.0,
            3.0, -4.0, 5.0,
            3.0, -4.0, 5.0,
        ]).unwrap();
        assert!(m.slae(&[1.0]).is_err());
    }
}
