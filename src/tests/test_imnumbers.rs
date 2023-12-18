#[doc(hidden)]
#[cfg(test)]
pub(crate) mod test_im_numbers {
    use crate::im_numbers::{ImExpression, ImNumber, ImOutput, XValue};

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
}
