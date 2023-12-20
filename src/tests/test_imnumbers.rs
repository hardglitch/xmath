#[cfg(test)]
pub(crate) mod test_im_numbers {
    use crate::im_numbers::{Im, ImExpression, ImNumber, ImOutput, XValue};

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
        let mut expr = 0.0.cast();
        expr.clean();
        let test_res = ImOutput { exprs: vec![] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_expr_cleaner1_neg() {
        let mut expr = 1.0.cast();
        expr.clean();
        let test_res = ImOutput { exprs: vec![] };
        assert_ne!(expr, test_res);
    }

    #[test]
    fn test_is_expr_zero1_pos() {
        let zero = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 0.0, im_pow: 0.0 }],
        }] }.exprs.first().unwrap().is_zero();
        assert!(zero);
    }

    #[test]
    fn test_is_expr_zero1_neg() {
        let non_zero = 0.0.cast().exprs.first().unwrap().is_zero();
        assert!(!non_zero);
    }

    #[test]
    fn test_is_expr_zero2_neg() {
        let non_zero = Im::new("0i").unwrap().exprs.first().unwrap().is_zero();
        assert!(!non_zero);
    }

    #[test]
    fn test_is_expr_one1_pos() {
        let one = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            pow: vec![ImNumber { real: 0.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
        }] }.exprs.first().unwrap().is_one();
        assert!(one);
    }

    #[test]
    fn test_is_expr_one1_neg() {
        let non_one = 0.0.cast().exprs.first().unwrap().is_one();
        assert!(!non_one);
    }

    #[test]
    fn test_is_expr_one2_neg() {
        let non_one = Im::new("0i").unwrap().exprs.first().unwrap().is_one();
        assert!(!non_one);
    }

    #[test]
    fn test_is_expr_one3_neg() {
        let non_one = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            pow: vec![ImNumber { real: 0.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 0.0, im_pow: 0.0 }],
        }] }.exprs.first().unwrap().is_one();
        assert!(!non_one);
    }

    #[test]
    fn test_imnum_let1_pos() {
        let expr = 1.0.cast();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let2_pos() {
        let expr = (-1.0).cast();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: -1.0, im_pow: 0.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let3_pos() {
        let expr = Im::new("2i").unwrap();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 2.0, im_pow: 1.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let4_pos() {
        let expr = Im::new("-2i").unwrap();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: -2.0, im_pow: 1.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let5_pos() {
        let expr = Im::new("2i_some").unwrap();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 2.0, im_pow: 1.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_let1_neg() {
        let expr = Im::new("2i").unwrap();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: -2.0, im_pow: 1.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
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
            mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
        }] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_add2_pos() {
        let expr = 1.0.cast() + 2.0.cast();
        let test_res = ImOutput { exprs: vec![ImExpression {
            elems: vec![ImNumber { real: 3.0, im_pow: 0.0 }],
            pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
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
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
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
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
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
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
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
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub1_pos() {
        let i_1i = Im::new("1i").unwrap();
        let i_2i = Im::new("2i").unwrap();
        let expr = i_2i - i_1i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub2_pos() {
        let i_1i = Im::new("1i").unwrap();
        let i_2i = Im::new("2i").unwrap();
        let expr = i_1i - i_2i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: -1.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub3_pos() {
        let i_1i = Im::new("-1i").unwrap();
        let i_2i = Im::new("2i").unwrap();
        let expr = i_2i - i_1i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 3.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub4_pos() {
        let i_1i = Im::new("-1i").unwrap();
        let i_2i = Im::new("2i").unwrap();
        let expr = (-1.0).cast() - i_2i - i_1i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: -1.0, im_pow: 0.0 }, ImNumber { real: -1.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub5_pos() {
        let i_2i = Im::new("2i").unwrap();
        let expr = i_2i - (1.0).cast();
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 2.0, im_pow: 1.0 }, ImNumber { real: -1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_sub6_pos() {
        let i_2i = Im::new("2i").unwrap();
        let expr = (i_2i.clone() - (1.0).cast()) - i_2i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: -1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul1_pos() {
        let i_1i = Im::new("-1i").unwrap();
        let i_2i = Im::new("2i").unwrap();
        let expr = i_2i * i_1i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 2.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul2_pos() {
        let i_1i = Im::new("1i").unwrap();
        let i_2i = Im::new("2i").unwrap();
        let expr = i_2i * i_1i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: -2.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul3_pos() {
        let i_1i = Im::new("-1i").unwrap();
        let i_2i = Im::new("2i").unwrap();
        let expr = i_2i.clone() * i_1i * i_2i.clone() * i_2i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: -8.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul4_pos() {
        let i_2i = Im::new("2i").unwrap();
        let expr = i_2i.clone() * 2.0.cast();
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 4.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul5_pos() {
        let expr = 2.0.cast() * (-2.0).cast();
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: -4.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul6_pos() {
        let i_2i = Im::new("2i").unwrap();
        let expr = (i_2i.clone() + 1.0.cast()) * i_2i;
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: -4.0, im_pow: 0.0 }, ImNumber { real: 2.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul7_pos() {
        let i_2i = Im::new("2i").unwrap();
        let expr = (i_2i.clone() - 1.0.cast()) * i_2i + 2.0.cast();
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: -2.0, im_pow: 0.0 }, ImNumber { real: -2.0, im_pow: 1.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul8_pos() {
        let i_2i = Im::new("2i").unwrap();
        let i_4i = Im::new("4i").unwrap();
        let expr = (i_2i.clone() - 2.0.cast()) * i_2i + 4.0.cast() + i_4i;
        let test_res = ImOutput { exprs: vec![] };
        assert_eq!(expr, test_res);
    }

    #[test]
    fn test_imnum_mul9_pos() {
        let expr = 0.0.cast() * (-2.0).cast();
        assert!(expr.exprs.is_empty());
    }

    #[test]
    fn test_imnum_mul10_pos() {
        let i_2i = Im::new("2i").unwrap();
        let expr = 0.0.cast() * (-2.0).cast() * i_2i;
        assert!(expr.exprs.is_empty());
    }

    #[test]
    fn test_imnum_mul11_pos() {
        let expr1 = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        let expr2 = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: -1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr1 * expr2, test_res);
    }

    #[test]
    fn test_imnum_mul12_pos() {
        let expr1 = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 2.0, im_pow: 1.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        let expr2 = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: -1.0, im_pow: 1.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 5.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr1 * expr2, test_res);
    }

    #[test]
    fn test_imnum_mul13_pos() {
        let expr1 = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        let expr2 = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 2.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        let test_res = ImOutput { exprs: vec![
            ImExpression {
                elems: vec![ImNumber { real: 2.0, im_pow: 0.0 }],
                pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
                mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
            }
        ]};
        assert_eq!(expr1 * expr2, test_res);
    }

    // #[test]
    // fn test_imnum_mul14_pos() {
    //     let expr1 = ImOutput { exprs: vec![
    //         ImExpression {
    //             elems: vec![ImNumber { real: 2.0, im_pow: 0.0 }],
    //             pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
    //             mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
    //         }
    //     ]};
    //     let expr2 = ImOutput { exprs: vec![
    //         ImExpression {
    //             elems: vec![ImNumber { real: 3.0, im_pow: 0.0 }],
    //             pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
    //             mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
    //         }
    //     ]};
    //     let test_res = ImOutput { exprs: vec![
    //         ImExpression {
    //             elems: vec![ImNumber { real: 2.0, im_pow: 0.0 }],
    //             pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
    //             mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
    //         },
    //         ImExpression {
    //             elems: vec![ImNumber { real: 3.0, im_pow: 0.0 }],
    //             pow: vec![ImNumber { real: 1.0, im_pow: 1.0 }],
    //             mul: vec![ImNumber { real: 1.0, im_pow: 0.0 }],
    //         }
    //     ]};
    //     assert_eq!(expr1 * expr2, test_res);
    // }
}
