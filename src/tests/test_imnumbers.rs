#[cfg(test)]
pub(crate) mod test_im_numbers {
    use crate::im_numbers::cast::ImValue;
    use crate::im_numbers::im_expression::ImExpression;
    use crate::im_numbers::im_number::ImNumber;
    use crate::im_numbers::im_output::{format_im_expr, ImOutput};

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
    fn test_expr_cleaner1_pos() {
        let mut expr = 0.r();
        expr.clean();
        assert!(expr.is_zero());
    }

    #[test]
    fn test_expr_cleaner1_neg() {
        let mut expr = 1.r();
        expr.clean();
        let test_res = ImOutput { exprs: vec![] };
        assert_ne!(expr, test_res);
    }

    #[test]
    fn test_is_expr_zero1_pos() {
        let zero = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 0.0, im_pow: 0.0 }],
            }]
        }.exprs.first().unwrap().is_mul_zero();
        assert!(zero);
    }

    #[test]
    fn test_is_expr_zero1_neg() {
        let non_zero = 0.r().exprs.first().unwrap().is_mul_zero();
        assert!(!non_zero);
    }

    #[test]
    fn test_is_expr_zero2_neg() {
        let non_zero = 0.i().exprs.first().unwrap().is_mul_zero();
        assert!(!non_zero);
    }

    #[test]
    fn test_is_expr_one1_pos() {
        let one = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 0.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }]
        }.exprs.first().unwrap().is_pow_zero();
        assert!(one);
    }

    #[test]
    fn test_is_expr_one1_neg() {
        let non_one = 0.r().exprs.first().unwrap().is_pow_zero();
        assert!(!non_one);
    }

    #[test]
    fn test_is_expr_one2_neg() {
        let non_one = 0.i().exprs.first().unwrap().is_pow_zero();
        assert!(!non_one);
    }

    #[test]
    fn test_is_expr_one3_neg() {
        let non_one = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 0.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 0.0, im_pow: 0.0 }],
            }]
        }.exprs.first().unwrap().is_pow_zero();
        assert!(non_one);
    }

    #[test]
    fn test_imnum_let1_pos() {
        let expr = 1.r();
        let test_res = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let2_pos() {
        let expr = (-1.0).r();
        let test_res = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: -1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let3_pos() {
        let test_res = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: 2.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }]
        };
        assert_eq!(2.i(), test_res);
    }

    #[test]
    fn test_imnum_let4_pos() {
        let expr = (-2).i();
        let test_res = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: -2.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let1_neg() {
        let test_res = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: -2.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }]
        };
        assert_ne!(2.i(), test_res);
    }

    #[test]
    fn test_imnum_add1_pos() {
        let expr = 1.i() + 2.i();
        let test_res = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: 3.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add2_pos() {
        let expr = 1.r() + 2.r();
        let test_res = ImOutput {
            exprs: vec![ImExpression {
                base: vec![ImNumber { real: 3.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add3_pos() {
        let expr = 1.i() + 1.r();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 1.0 }, ImNumber { real: 1.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add4_pos() {
        let expr = 1.r() + 1.i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 0.0 }, ImNumber { real: 1.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add5_pos() {
        let expr = 1.r() + (-1).i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 0.0 }, ImNumber { real: -1.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add6_pos() {
        let expr = (-1).i() - 1.i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -2.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add7_pos() {
        let expr1 = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        let expr2 = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 2.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr1 + expr2, test_res);
    }

    #[test]
    fn test_imnum_sub1_pos() {
        let expr = 2.i() - 1.i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub2_pos() {
        let expr = 1.i() - 2.i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -1.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub3_pos() {
        let expr = 2.i() - (-1).i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 3.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub4_pos() {
        let expr = (-1).r() - 2.i() - (-1).i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -1.0, im_pow: 0.0 }, ImNumber { real: -1.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub5_pos() {
        let expr = 2.i() - (1.0).r();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 2.0, im_pow: 1.0 }, ImNumber { real: -1.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub6_pos() {
        let expr = (2.i() - (1.0).r()) - 2.i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -1.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul1_pos() {
        let expr = 2.i() * (-1).i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 2.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul2_pos() {
        let expr = 2.i() * 1.i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -2.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul3_pos() {
        let expr = 2.i() * (-1).i() * 2.i() * 2.i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -8.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul4_pos() {
        let expr = 2.i() * 2.r();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 4.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul5_pos() {
        let expr = 2.r() * (-2.0).r();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -4.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul6_pos() {
        let expr = (2.i() + 1.r()) * 2.i();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -4.0, im_pow: 0.0 }, ImNumber { real: 2.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul7_pos() {
        let expr = (2.i() - 1.r()) * 2.i() + 2.r();
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: -2.0, im_pow: 0.0 }, ImNumber { real: -2.0, im_pow: 1.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
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
        let expr1 = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        let expr2 = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: -1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr1 * expr2, test_res);
    }

    #[test]
    fn test_imnum_mul12_pos() {
        let expr1 = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 2.0, im_pow: 1.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        let expr2 = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: -1.0, im_pow: 1.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr1 * expr2, test_res);
    }

    #[test]
    fn test_imnum_mul13_pos() {
        let expr1 = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        let expr2 = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 2.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        let test_res = ImOutput {
            exprs: vec![
                ImExpression {
                    base: vec![ImNumber { real: 2.0, im_pow: 0.0 }],
                    pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                    mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                }
            ]
        };
        assert_eq!(expr1 * expr2, test_res);
    }

    #[test]
    fn test_collect1_pos() {
        let expr = (1.r() + 2.i()) * (3.r() + 1.i()) + 4.i() - (4.i() + 6.r()); // 7i - 5
        assert_eq!("(7i-5)", format_im_expr(expr.exprs.as_slice()));
    }
}