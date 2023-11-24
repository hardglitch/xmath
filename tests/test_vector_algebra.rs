#[cfg(test)]
mod test_vector_algebra {
    use math::vector_algebra::{cos_alpha, Vector3D, vector_multiplication};

    #[test]
    fn test_vector_algebra_cos_alpha1_pos() {
        let a = Vector3D{x:3.0, y:4.0, z:7.0};
        let b = Vector3D{x:4.0, y:8.0, z:5.0};
        let res = cos_alpha(&a, &b).unwrap();
        let res_test = 0.896224025128485;
        assert_eq!(res, res_test);
    }

    #[test]
    fn test_vector_algebra_cos_alpha1_neg() {
        let a = Vector3D{x:0.0, y:0.0, z:0.0};
        let b = Vector3D{x:4.0, y:8.0, z:5.0};
        let res = cos_alpha(&a, &b).unwrap();
        assert!(res.is_nan());
    }

    #[test]
    fn test_vector_algebra_vector_multiplication1_pos() {
        let a = Vector3D{x:1.0, y:3.0, z:7.0};
        let b = Vector3D{x:7.0, y:8.0, z:6.0};
        let res = vector_multiplication(&a, &b);
        let res_test = Vector3D{x:-38.0, y:43.0, z:-13.0};
        assert_eq!(res, res_test);
    }
}
