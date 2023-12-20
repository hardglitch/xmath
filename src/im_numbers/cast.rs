use crate::im_numbers::im_expression::ImExpression;
use crate::im_numbers::im_number::ImNumber;
use crate::im_numbers::im_output::ImOutput;

pub trait ImValue<T> {
    fn r(self) -> T;
    fn i(self) -> T;
}

impl ImValue<ImOutput> for f64 {
    fn r(self) -> ImOutput {
        let im_num = ImNumber::new(self, 0.0);
        let expr_one = ImNumber::new(1.0, 0.0);
        let im_expr = ImExpression { base: vec![im_num], pow: vec![expr_one.clone()], mul: vec![expr_one] };
        ImOutput { exprs: vec![im_expr] }
    }

    fn i(self) -> ImOutput {
        let im_num = ImNumber::new(self, 1.0);
        let expr_one = ImNumber::new(1.0, 0.0);
        let im_expr = ImExpression { base: vec![im_num], pow: vec![expr_one.clone()], mul: vec![expr_one] };
        ImOutput { exprs: vec![im_expr] }
    }
}

impl ImValue<ImOutput> for i32 {
    fn r(self) -> ImOutput {
        let im_num = ImNumber::new(self as f64, 0.0);
        let expr_one = ImNumber::new(1.0, 0.0);
        let im_expr = ImExpression { base: vec![im_num], pow: vec![expr_one.clone()], mul: vec![expr_one] };
        ImOutput { exprs: vec![im_expr] }
    }

    fn i(self) -> ImOutput {
        let im_num = ImNumber::new(self as f64, 1.0);
        let expr_one = ImNumber::new(1.0, 0.0);
        let im_expr = ImExpression { base: vec![im_num], pow: vec![expr_one.clone()], mul: vec![expr_one] };
        ImOutput { exprs: vec![im_expr] }
    }
}
