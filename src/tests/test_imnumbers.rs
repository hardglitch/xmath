#[cfg(test)]
pub(crate) mod test_im_numbers {

    #[cfg(test)]
    mod test_let {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

        #[test]
        fn test_let1_pos() {
            let test_res = Im::new(1.0, 0.0);
            assert_eq!(1.r(), test_res);
        }

        #[test]
        fn test_let2_pos() {
            let test_res = Im::new(-1.0, 0.0);
            assert_eq!((-1.0).r(), test_res);
        }

        #[test]
        fn test_let3_pos() {
            let test_res = Im::new(2.0, 1.0);
            assert_eq!(2.i(), test_res);
        }

        #[test]
        fn test_let4_pos() {
            let test_res = Im::new(-2.0, 1.0);
            assert_eq!((-2).i(), test_res);
        }

        #[test]
        fn test_let1_neg() {
            let test_res = Im::new(-2.0, 1.0);
            assert_ne!(2.i(), test_res);
        }
    }

    #[cfg(test)]
    mod test_zero {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

        #[test]
        fn test_expr_is_zero1_pos() {
            assert!(0.r().is_zero());
        }

        #[test]
        fn test_expr_is_zero2_pos() {
            assert!(0.i().is_zero());
        }

        #[test]
        fn test_expr_is_zero3_pos() {
            let expr = 0.i() + 0.i();
            assert!(expr.is_zero());
        }

        #[test]
        fn test_expr_is_zero4_pos() {
            let expr = 0.i() + 0.r();
            assert!(expr.is_zero());
        }

        #[test]
        fn test_expr_is_zero5_pos() {
            let expr = 0.r() + 0.r();
            assert!(expr.is_zero());
        }

        #[test]
        fn test_expr_is_zero6_pos() {
            let expr = 0.r() + 0.i();
            assert!(expr.is_zero());
        }

        #[test]
        fn test_expr_is_zero7_pos() {
            let expr = (-1).r() + 1.r();
            assert!(expr.is_zero());
        }

        #[test]
        fn test_expr_is_zero8_pos() {
            let expr = (-1).i() + 1.i();
            assert!(expr.is_zero());
        }

        #[test]
        fn test_expr_is_zero9_pos() {
            let expr = Im::default();
            assert!(expr.is_zero());
        }

        #[test]
        fn test_expr_is_zero10_pos() {
            assert!(0.r() == 0.i());
        }

        #[test]
        fn test_expr_is_zero11_pos() {
            assert_eq!(0.r(), 0.i());
        }

        #[test]
        fn test_expr_is_zero1_neg() {
            let expr = 0.i() + 1.r();
            assert!(!expr.is_zero());
        }
    }

    #[cfg(test)]
    mod test_im_pow_fixer {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

        #[test]
        fn test_pair_checker1_pos() {
            let mut im_num = 1.i();
            im_num.im_pow_fixer();
            let test_res = 1.i();
            assert_eq!(im_num, test_res);
        }

        #[test]
        fn test_pair_checker2_pos() {
            let mut im_num = 1.r();
            im_num.im_pow_fixer();
            let test_res = 1.r();
            assert_eq!(im_num, test_res);
        }

        #[test]
        fn test_pair_checker3_pos() {

            let mut im_num = Im::new(1.0, 2.0);
            im_num.im_pow_fixer();
            let test_res = (-1).r();
            assert_eq!(im_num, test_res);
        }

        #[test]
        fn test_pair_checker4_pos() {
            let mut im_num = Im::new(3.0, 5.0);
            im_num.im_pow_fixer();
            let test_res = (3).i();
            assert_eq!(im_num, test_res);
        }

        #[test]
        fn test_pair_checker5_pos() {
            let mut im_num = Im::new(3.0, 4.0);
            im_num.im_pow_fixer();
            let test_res = (3).r();
            assert_eq!(im_num, test_res);
        }

        #[test]
        fn test_pair_checker6_pos() {
            let mut im_num = Im::new(-3.0, 3.0);
            im_num.im_pow_fixer();
            let test_res = (3).i();
            assert_eq!(im_num, test_res);
        }
    }

    #[cfg(test)]
    mod test_neg {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

        #[test]
        fn test_neg1_pos() {
            let mut expr = 1.i();
            unsafe { expr.neg(); }
            let test_res = (-1).i();
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_neg2_pos() {
            let mut expr = (-1).i();
            unsafe { expr.neg(); }
            let test_res = (1).i();
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_neg3_pos() {
            let mut expr = (-1).r();
            unsafe { expr.neg(); }
            let test_res = (1).r();
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_neg4_pos() {
            let mut expr = 1.r();
            unsafe { expr.neg(); }
            let test_res = (-1).r();
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_neg5_pos() {
            let mut expr = 3.r() + 2.i();
            unsafe { expr.neg(); }
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(-3.0, 0.0));
            test_res.push_in_mixed_base(Im::new(-2.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_neg6_pos() {
            let mut expr = (3.r() + 2.i()).pow(1.i());
            unsafe { expr.neg(); }

            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(3.0, 0.0));
            test_res.push_in_mixed_base(Im::new(2.0, 1.0));
            test_res.push_in_mixed_pow(Im::new(1.0, 1.0));
            test_res.push_in_mixed_mul(Im::new(-1.0, 0.0));

            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_neg7_pos() {
            let mut expr = (3.r() + 2.i()).pow(2.i()) * 2.i();
            unsafe { expr.neg(); }

            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(3.0, 0.0));
            test_res.push_in_mixed_base(Im::new(2.0, 1.0));
            test_res.push_in_mixed_pow(Im::new(2.0, 1.0));
            test_res.push_in_mixed_mul(Im::new(-2.0, 1.0));

            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_neg1_neg() {
            let mut expr = 1.i();
            unsafe { expr.neg(); }
            let test_res = (-1).r();
            assert_ne!(expr, test_res);
        }

        #[test]
        fn test_neg2_neg() {
            let mut expr = 1.r();
            unsafe { expr.neg(); }
            let test_res = (-1).i();
            assert_ne!(expr, test_res);
        }
    }

    #[cfg(test)]
    mod test_abs {
        use crate::im::cast::ImValue;
        use crate::im::core::Sign;

        #[test]
        fn test_abs1_pos() {
            let sign = unsafe { 1.r().is_equal_by_abs(&(-1).r()) };
            assert_eq!(sign, Sign::Minus);
        }

        #[test]
        fn test_abs2_pos() {
            let sign = unsafe { 1.r().is_equal_by_abs(&(1).r()) };
            assert_eq!(sign, Sign::Plus);
        }

        #[test]
        fn test_abs3_pos() {
            let sign = unsafe { 1.r().is_equal_by_abs(&(-2).r()) };
            assert_eq!(sign, Sign::None);
        }
    }

    #[cfg(test)]
    mod test_add {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

        #[test]
        fn test_add1_pos() {
            let expr = 1.i() + 2.i();
            let test_res = Im::new(3.0, 1.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_add2_pos() {
            let expr = 1.r() + 2.r();
            let test_res = Im::new(3.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_add3_pos() {
            let expr = 1.i() + 1.r();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(1.0, 1.0));
            test_res.push_in_mixed_base(Im::new(1.0, 0.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_add4_pos() {
            let expr = 1.r() + 1.i();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(1.0, 0.0));
            test_res.push_in_mixed_base(Im::new(1.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_add5_pos() {
            let expr = 1.r() + (-1).i();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(1.0, 0.0));
            test_res.push_in_mixed_base(Im::new(-1.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_add6_pos() {
            let expr = (-1).r() + (-1).i();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(-1.0, 0.0));
            test_res.push_in_mixed_base(Im::new(-1.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_add7_pos() {
            let expr = 1.r() + 1.i() + 2.r();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(3.0, 0.0));
            test_res.push_in_mixed_base(Im::new(1.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_add8_pos() {
            let expr = 1.i() + 1.r() + 2.i() + 2.r();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(3.0, 1.0));
            test_res.push_in_mixed_base(Im::new(3.0, 0.0));
            assert_eq!(expr, test_res);
        }
    }

    #[cfg(test)]
    mod test_sub {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

        #[test]
        fn test_sub1_pos() {
            let expr = 1.i() - 2.i();
            let test_res = Im::new(-1.0, 1.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_sub2_pos() {
            let expr = 1.r() - 2.r();
            let test_res = Im::new(-1.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_sub3_pos() {
            let expr = 1.i() - 1.r();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(1.0, 1.0));
            test_res.push_in_mixed_base(Im::new(-1.0, 0.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_sub4_pos() {
            let expr = 1.r() - 1.i();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(1.0, 0.0));
            test_res.push_in_mixed_base(Im::new(-1.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_sub5_pos() {
            let expr = 1.r() - (-1).i();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(1.0, 0.0));
            test_res.push_in_mixed_base(Im::new(1.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_sub6_pos() {
            let expr = (-1).r() - (-1).i();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(-1.0, 0.0));
            test_res.push_in_mixed_base(Im::new(1.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_sub7_pos() {
            let expr = 1.r() - 1.i() - 2.r();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(-1.0, 0.0));
            test_res.push_in_mixed_base(Im::new(-1.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_sub8_pos() {
            let expr = 1.i() - 1.r() - 2.i() - 2.r();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(-1.0, 1.0));
            test_res.push_in_mixed_base(Im::new(-3.0, 0.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_sub9_pos() {
            let expr = 1.i() - (-1).r() + 2.i() - 2.r() + (-1).i();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(2.0, 1.0));
            test_res.push_in_mixed_base(Im::new(-1.0, 0.0));
            assert_eq!(expr, test_res);
        }
    }

    #[cfg(test)]
    mod test_other {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

        #[test]
        fn test_is_mixed_base_logic1_pos() {
            let mut expr = Im::default();
            expr.push_in_mixed_base(1.r());
            expr.mixed_base = None;
            assert!(!expr.is_mixed_base_logic(&expr));
        }

        #[test]
        fn test_is_mixed_base_logic1_neg() {
            let expr = Im::default();
            assert!(!expr.is_mixed_base_logic(&expr));
        }

        #[test]
        fn test_collect1_pos() {
            let mut expr = Im::default();
            expr.push_in_mixed_base(Im::new(1.0, 1.0));
            expr.push_in_mixed_base(Im::new(1.0, 0.0));
            expr.push_in_mixed_base(Im::new(-1.0, 0.0));
            expr.push_in_mixed_base(Im::new(1.0, 1.0));
            unsafe { expr.collect(); }
            assert_eq!(expr, 2.i());
        }
    }

    #[cfg(test)]
    mod test_mul {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

        #[test]
        fn test_mul1_pos() {
            let expr = 2.i() * (-1).i();
            let test_res = Im::new(2.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_mul2_pos() {
            let expr = 2.i() * 1.i();
            let test_res = Im::new(-2.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_mul3_pos() {
            let expr = 2.i() * (-1).i() * 2.i() * 2.i();
            let test_res = Im::new(-8.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_mul4_pos() {
            let expr = 2.i() * 2.r();
            let test_res = Im::new(4.0, 1.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_mul5_pos() {
            let expr = 2.r() * (-2.0).r();
            let test_res = Im::new(-4.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_mul6_pos() {
            let expr = (2.i() + 1.r()) * 2.i();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(2.0, 1.0));
            test_res.push_in_mixed_base(Im::new(-4.0, 0.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_mul7_pos() {
            let expr = (2.i() - 1.r()) * 2.i() + 2.r();
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(-2.0, 1.0));
            test_res.push_in_mixed_base(Im::new(-2.0, 0.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_mul8_pos() {
            let expr = (2.i() - 2.r()) * 2.i() + 4.r() + 4.i();
            assert!(expr.is_zero());
        }

        #[test]
        fn test_mul9_pos() {
            let expr = 0.r() * (-2.0).r();
            assert!(expr.is_zero());
        }

        #[test]
        fn test_mul10_pos() {
            let expr = 0.r() * (-2.0).r() * 2.i();
            assert!(expr.is_zero());
        }


        #[test]
        fn test_mul11_pos() {
            let expr = (1.r() + 1.i()) * (1.i() + 1.r());
            assert_eq!(expr, 2.i());
        }

        #[test]
        fn test_mul12_pos() {
            let expr1 = (1.r() + 2.i()) * (3.r() + 1.i()); // 1 + 7i
            let expr2 = 4.i() - (4.i() + 6.r()); // 6
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(7.0, 0.0));
            test_res.push_in_mixed_base(Im::new(7.0, 1.0));
            assert_eq!(expr1 + expr2, test_res);
        }
    }

    #[cfg(test)]
    mod test_pow {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

        #[test]
        fn test_pow1_pos() {
            let expr = 1.r().pow(1.r());
            let test_res = Im::new(1.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow2_pos() {
            let expr = 1.r().pow(0.r());
            let test_res = Im::new(1.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow3_pos() {
            let expr = 0.r().pow(1.r());
            let test_res = Im::new(0.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow4_pos() {
            let expr = 0.r().pow((-1).r());
            let test_res = Im::new(0.0, 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow5_pos() {
            let expr = 2.r().pow(0.5.r());
            let test_res = Im::new(2.0_f64.sqrt(), 0.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow6_pos() {
            let expr = (1.r() + 2.i()).pow(2.r());
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(-3.0, 0.0));
            test_res.push_in_mixed_base(Im::new(4.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow7_pos() {
            let expr = 2.i().pow(3.r());
            let test_res = Im::new(-8.0, 1.0);
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow8_pos() {
            let expr = 2.i().pow(2.i());
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(2.0, 1.0));
            test_res.push_in_mixed_pow(Im::new(2.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow9_pos() {
            let expr = 2.r().pow(1.i()) - 3.r().pow(1.i()); // 2^i - 3^i

            let mut expr1 = Im::default();
            expr1.push_in_mixed_base(Im::new(2.0, 0.0));
            expr1.push_in_mixed_pow(Im::new(1.0, 1.0));

            let mut expr2 = Im::default();
            expr2.push_in_mixed_base(Im::new(3.0, 0.0));
            expr2.push_in_mixed_pow(Im::new(1.0, 1.0));
            expr2.push_in_mixed_mul(Im::new(-1.0, 0.0));

            let mut test_expr = Im::default();
            test_expr.push_in_mixed_base(expr1);
            test_expr.push_in_mixed_base(expr2);

            assert_eq!(expr, test_expr);
        }

        #[test]
        fn test_pow10_pos() {
            let expr = 2.i().pow((-1).i());
            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(2.0, 1.0));
            test_res.push_in_mixed_pow(Im::new(-1.0, 1.0));
            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow11_pos() {
            let expr = (2.i() + 1.r()).pow((-1).i());

            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(2.0, 1.0));
            test_res.push_in_mixed_base(Im::new(1.0, 0.0));
            test_res.push_in_mixed_pow(Im::new(-1.0, 1.0));

            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow12_pos() {
            let expr = (2.i() + 1.r()).pow((-1).i()) * 2.r();

            let mut test_res = Im::default();
            test_res.push_in_mixed_base(Im::new(2.0, 1.0));
            test_res.push_in_mixed_base(Im::new(1.0, 0.0));
            test_res.push_in_mixed_pow(Im::new(-1.0, 1.0));
            test_res.push_in_mixed_mul(Im::new(2.0, 0.0));

            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow13_pos() {
            let expr = (2.i() + 1.r()).pow((-1).i()) - 1.r();

            let mut expr1 = Im::default();
            expr1.push_in_mixed_base(Im::new(2.0, 1.0));
            expr1.push_in_mixed_base(Im::new(1.0, 0.0));
            expr1.push_in_mixed_pow(Im::new(-1.0, 1.0));

            let expr2 = Im::new(-1.0, 0.0);

            let mut test_res = Im::default();
            test_res.push_in_mixed_base(expr1);
            test_res.push_in_mixed_base(expr2);

            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow14_pos() {
            let expr = (2.i() - 1.r()).pow(2.r() + 1.i()) - (4.i() - 2.r()).pow(2.r() + 1.i());
            // (2i-1)^(2+i) - (4i-2)^(2+i)

            let mut expr1 = Im::default();
            expr1.push_in_mixed_base(Im::new(2.0, 1.0));
            expr1.push_in_mixed_base(Im::new(-1.0, 0.0));
            expr1.push_in_mixed_pow(Im::new(2.0, 0.0));
            expr1.push_in_mixed_pow(Im::new(1.0, 1.0));

            let mut expr2 = Im::default();
            expr2.push_in_mixed_mul(Im::new(-1.0, 0.0));
            expr2.push_in_mixed_base(Im::new(4.0, 1.0));
            expr2.push_in_mixed_base(Im::new(-2.0, 0.0));
            expr2.push_in_mixed_pow(Im::new(2.0, 0.0));
            expr2.push_in_mixed_pow(Im::new(1.0, 1.0));

            let mut test_res = Im::default();
            test_res.push_in_mixed_base(expr1);
            test_res.push_in_mixed_base(expr2);

            assert_eq!(expr, test_res);
        }

        #[test]
        fn test_pow15_pos() {
            assert_eq!(1.r().pow(1.i()), 1.r())
        }

        #[test]
        fn test_pow16_pos() {
            assert_eq!(1.r().pow(1.i()) * 1.i().pow(1.i()), 1.i().pow(1.i()))
        }

        // for pow :  5 * 1/5, 5^2 / 2^-1
    }

    #[cfg(test)]
    mod test_div {
        use crate::im::cast::ImValue;
        use crate::im::core::Im;

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
        fn test_div1_neg() {
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
            let expr = 5.r() * 1.r()/5.r();
            let test_res = Im::new(1.0, 0.0);
            assert_eq!(expr, test_res);
        }
    }

    #[cfg(test)]
    mod test_format {
        use crate::im::cast::ImValue;

        #[test]
        fn test_format1_pos() {
            assert_eq!("1", 1.r().format());
        }

        #[test]
        fn test_format2_pos() {
            assert_eq!("-1", (-1).r().format());
        }

        #[test]
        fn test_format3_pos() {
            assert_eq!("i", (1).i().format());
        }

        #[test]
        fn test_format4_pos() {
            assert_eq!("-i", (-1).i().format());
        }

        #[test]
        fn test_format5_pos() {
            assert_eq!("0", 0.i().format());
        }

        #[test]
        fn test_format6_pos() {
            assert_eq!("0", (-0).r().format());
        }

        #[test]
        fn test_format7_pos() {
            assert_eq!("-1/8i", 2.i().pow((-3).r()).format());
        }

        #[test]
        fn test_format8_pos() {
            assert_eq!("0.25", 2.r().pow((-2).r()).format());
        }

        #[test]
        fn test_format1_neg() {
            assert_eq!("None", (1.r() / 0.r()).format());
        }

        //     #[test]
        //     fn test_div7_pos() {
        //         let expr = (4.r() - 7.i()) / (6.i() - 7.i());
        //         assert_eq!("(7-4/i)", format_im_expr(expr.exprs.as_slice()));
        //     }
        //
        //     #[test]
        //     fn test_div8_pos() {
        //         let expr = (4.r() - 7.i()) / (6.r() - 7.i());
        //         assert_eq!("(4-7i)/(6-7i)", format_im_expr(expr.exprs.as_slice()));
        //     }
        //
    }
}