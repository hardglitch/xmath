#[cfg(test)]
pub(crate) mod test_im_numbers {
    use crate::im_numbers::cast::ImValue;
    use crate::im_numbers::im_expression::ImExpression;
    use crate::im_numbers::im_number::ImNumber;
    use crate::im_numbers::im_output::{format_im_expr, Im};

    #[test]
    fn test_new_im_expr() {
        ImExpression::new(1.0, 0.0);
    }

    #[test]
    fn test_pair_checker1_pos() {
        let mut im_num = ImNumber { real: 1.0, im_pow: 1.0 };
        im_num.pair_checker();
        let test_res = ImNumber { real: 1.0, im_pow: 1.0 };
        assert_eq!(im_num, test_res);
    }

    #[test]
    fn test_pair_checker2_pos() {
        let mut im_num = ImNumber { real: 1.0, im_pow: 0.0 };
        im_num.pair_checker();
        let test_res = ImNumber { real: 1.0, im_pow: 0.0 };
        assert_eq!(im_num, test_res);
    }

    #[test]
    fn test_pair_checker3_pos() {
        let mut im_num = ImNumber { real: 1.0, im_pow: 2.0 };
        im_num.pair_checker();
        let test_res = ImNumber { real: -1.0, im_pow: 0.0 };
        assert_eq!(im_num, test_res);
    }

    #[test]
    fn test_pair_checker4_pos() {
        let mut im_num = ImNumber { real: 3.0, im_pow: 5.0 };
        im_num.pair_checker();
        let test_res = ImNumber { real: 3.0, im_pow: 1.0 };
        assert_eq!(im_num, test_res);
    }

    #[test]
    fn test_pair_checker5_pos() {
        let mut im_num = ImNumber { real: 3.0, im_pow: 4.0 };
        im_num.pair_checker();
        let test_res = ImNumber { real: 3.0, im_pow: 0.0 };
        assert_eq!(im_num, test_res);
    }

    #[test]
    fn test_pair_checker6_pos() {
        let mut im_num = ImNumber { real: 3.0, im_pow: 3.0 };
        im_num.pair_checker();
        let test_res = ImNumber { real: -3.0, im_pow: 1.0 };
        assert_eq!(im_num, test_res);
    }

    #[test]
    fn test_expr_is_zero1_pos() {
        assert!(0.r().is_zero());
    }

    #[test]
    fn test_expr_is_zero1_neg() {
        assert!(!1.r().is_zero());
    }

    #[test]
    fn test_imnum_let1_pos() {
        let test_res = Im::new(1.0, 0.0);
        assert_eq!(1.r(), test_res);
    }

    #[test]
    fn test_imnum_let2_pos() {
        let test_res = Im::new(-1.0, 0.0);
        assert_eq!((-1.0).r(), test_res);
    }

    #[test]
    fn test_imnum_let3_pos() {
        let test_res = Im::new(2.0, 1.0);
        assert_eq!(2.i(), test_res);
    }

    #[test]
    fn test_imnum_let4_pos() {
        let test_res = Im::new(-2.0, 1.0);
        assert_eq!((-2).i(), test_res);
    }

    #[test]
    fn test_imnum_let1_neg() {
        let test_res = Im::new(-2.0, 1.0);
        assert_ne!(2.i(), test_res);
    }

    #[test]
    fn test_imnum_add1_pos() {
        let expr = 1.i() + 2.i();
        let test_res = Im::new(3.0, 1.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add2_pos() {
        let expr = 1.r() + 2.r();
        let test_res = Im::new(3.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add3_pos() {
        let expr = 1.i() + 1.r();
        let test_res = Im {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 1.0 }, ImNumber { real: 1.0, im_pow: 0.0 }],
                    pow: Default::default(),
                    mul: Default::default(),
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add4_pos() {
        let expr = 1.r() + 1.i();
        let test_res = Im {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 0.0 }, ImNumber { real: 1.0, im_pow: 1.0 }],
                    pow: Default::default(),
                    mul: Default::default(),
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add5_pos() {
        let expr = 1.r() + (-1).i();
        let test_res = Im {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 0.0 }, ImNumber { real: -1.0, im_pow: 1.0 }],
                    pow: Default::default(),
                    mul: Default::default(),
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add6_pos() {
        let expr = (-1).i() - 1.i();
        let test_res = Im::new(-2.0, 1.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub1_pos() {
        let expr = 2.i() - 1.i();
        let test_res = Im::new(1.0, 1.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub2_pos() {
        let expr = 1.i() - 2.i();
        let test_res = Im::new(-1.0, 1.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub3_pos() {
        let expr = 2.i() - (-1).i();
        let test_res = Im::new(3.0, 1.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub4_pos() {
        let expr = (-1).r() - 2.i() - (-1).i();
        let test_res = Im {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -1.0, im_pow: 0.0 }, ImNumber { real: -1.0, im_pow: 1.0 }],
                    pow: Default::default(),
                    mul: Default::default(),
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub5_pos() {
        let expr = 2.i() - (1.0).r();
        let test_res = Im {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 2.0, im_pow: 1.0 }, ImNumber { real: -1.0, im_pow: 0.0 }],
                    pow: Default::default(),
                    mul: Default::default(),
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub6_pos() {
        let expr = (2.i() - (1.0).r()) - 2.i();
        let test_res = Im::new(-1.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul1_pos() {
        let expr = 2.i() * (-1).i();
        let test_res = Im::new(2.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul2_pos() {
        let expr = 2.i() * 1.i();
        let test_res = Im::new(-2.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul3_pos() {
        let expr = 2.i() * (-1).i() * 2.i() * 2.i();
        let test_res = Im::new(-8.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul4_pos() {
        let expr = 2.i() * 2.r();
        let test_res = Im::new(4.0, 1.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul5_pos() {
        let expr = 2.r() * (-2.0).r();
        let test_res = Im::new(-4.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul6_pos() {
        let expr = (2.i() + 1.r()) * 2.i();
        let test_res = Im {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 2.0, im_pow: 1.0 }, ImNumber { real: -4.0, im_pow: 0.0 }],
                    pow: Default::default(),
                    mul: Default::default(),
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul7_pos() {
        let expr = (2.i() - 1.r()) * 2.i() + 2.r();
        let test_res = Im {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -2.0, im_pow: 1.0 }, ImNumber { real: -2.0, im_pow: 0.0 }],
                    pow: Default::default(),
                    mul: Default::default(),
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul8_pos() {
        let expr = (2.i() - 2.r()) * 2.i() + 4.r() + 4.i();
        assert!(expr.is_zero());
    }

    #[test]
    fn test_imnum_mul9_pos() {
        let expr = 0.r() * (-2.0).r();
        assert!(expr.is_zero());
    }

    #[test]
    fn test_imnum_mul10_pos() {
        let expr = 0.r() * (-2.0).r() * 2.i();
        assert!(expr.is_zero());
    }

    #[test]
    fn test_imnum_mul11_pos() {
        let expr = 5.r() * 1.r()/5.r();
        let test_res = Im::new(1.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_collect1_pos() {
        let expr = (1.r() + 2.i()) * (3.r() + 1.i()) + 4.i() - (4.i() + 6.r());
        assert_eq!("(7i-5)", format_im_expr(expr.exprs.as_slice()));
    }

    #[test]
    fn test_div1_pos() {
        let expr = 1.i() / 1.i();
        let test_res = Im::new(1.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_div2_pos() {
        let expr = (-1).i() / 1.i();
        let test_res = Im::new(-1.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_div3_pos() {
        let expr = 0.i() / 1.i();
        let test_res = Im::new(0.0, 0.0);
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_div4_pos() {
        let expr = 1.i() / 0.i();
        assert!(expr.is_none());
    }

    #[test]
    fn test_div5_pos() {
        let expr = 1.i() / 0.i() * (4.i() + 7.r());
        assert!(expr.is_none());
    }

    #[test]
    fn test_div6_pos() {
        let expr = (4.r() - 7.i()) / (7.i() - 7.i());
        assert!(expr.is_none());
    }

    #[test]
    fn test_div7_pos() {
        let expr = (4.r() - 7.i()) / (6.i() - 7.i());
        assert_eq!("(7-4/i)", format_im_expr(expr.exprs.as_slice()));
    }

    #[test]
    fn test_div8_pos() {
        let expr = (4.r() - 7.i()) / (6.r() - 7.i());
        assert_eq!("(4-7i)/(6-7i)", format_im_expr(expr.exprs.as_slice()));
    }


    // for pow :  1^i * i^i, 5 * 1/5, 5^2 / 2^-1
}