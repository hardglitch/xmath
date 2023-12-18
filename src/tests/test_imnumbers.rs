#[doc(hidden)]
#[cfg(test)]
pub(crate) mod test_im_numbers {
    use crate::im_numbers::{Im, ImExpression, ImNumber, ImOutput, XValue};

    #[test]
    fn test_imnum_let1_pos() {
        let expr = 1.0.cast();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: 1.0,
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let2_pos() {
        let expr = (-1.0).cast();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: -1.0, im_pow: 0.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: 1.0,
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let3_pos() {
        let expr = Im::new("2i").unwrap();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 2.0, im_pow: 1.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: 1.0,
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let4_pos() {
        let expr = Im::new("-2i").unwrap();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: -2.0, im_pow: 1.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: 1.0,
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let5_pos() {
        let expr = Im::new("2i_some").unwrap();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 2.0, im_pow: 1.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: 1.0,
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let1_neg() {
        let expr = Im::new("2i").unwrap();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: -2.0, im_pow: 1.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: 1.0,
        }] };
        assert_ne!(expr, test_res);
    }

    #[test]
    fn test_imnum_let2_neg() {
        let expr = Im::new("some_2i");
        assert!(expr.is_err());
    }

    #[test]
    fn test_imnum_add1_pos() {
        let i_2i = Im::new("2i").unwrap();
        let i_1i = Im::new("1i").unwrap();
        let expr = i_1i + i_2i;
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 3.0, im_pow: 1.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: 1.0,
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add2_pos() {
        let expr = 1.0.cast() + 2.0.cast();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 3.0, im_pow: 0.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: 1.0,
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add3_pos() {
        let i_1i = Im::new("1i").unwrap();
        let expr = i_1i + 1.0.cast();
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 1.0, im_pow: 1.0 }, ImNumber { real: 1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: 1.0,
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add4_pos() {
        let i_1i = Im::new("1i").unwrap();
        let expr = 1.0.cast() + i_1i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 1.0, im_pow: 0.0 }, ImNumber { real: 1.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: 1.0,
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add5_pos() {
        let i_1i = Im::new("-1i").unwrap();
        let expr = 1.0.cast() + i_1i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 1.0, im_pow: 0.0 }, ImNumber { real: -1.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: 1.0,
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add6_pos() {
        let i_1i = Im::new("-1i").unwrap();
        let expr = i_1i.clone() + i_1i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: -2.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: 1.0,
            }
        ]};
        assert_eq!(expr, test_res);
    }
}
