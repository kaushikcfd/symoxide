use std::rc::Rc;
use crate::primitives::{BinaryOpType, UnaryOpType, Expression, ScalarT};


// {{{ FoldMapper

pub trait FoldMapper {
    type Output;

    fn visit(&self, expr: &Expression) -> Self::Output {
        match expr {
            Expression::Scalar(s)          => self.map_scalar(&s),
            Expression::Variable(name)     => self.map_variable(name.to_string()),
            Expression::UnaryOp(op, x)     => self.map_unary_op(op.clone(), &x),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r),
            Expression::Call(call, params) => self.map_call(&call, &params),
        }
    }

    fn map_scalar(&self, value: &ScalarT) -> Self::Output;
    fn map_variable(&self, name: String) -> Self::Output;
    fn map_unary_op(&self, op: UnaryOpType, x: &Expression) -> Self::Output;
    fn map_binary_op(&self, left: &Expression, op: BinaryOpType, right: &Expression) -> Self::Output;
    fn map_call(&self, call: &Expression, params: &Vec<Rc<Expression>>) -> Self::Output;
}

// }}}


// {{{ FoldMapperWithContext

pub trait FoldMapperWithContext {
    type Context;
    type Output;


    fn visit(&self, expr: &Expression, context: &Self::Context) -> Self::Output {
        match expr {
            Expression::Scalar(s)          => self.map_scalar(&s, context),
            Expression::Variable(name)     => self.map_variable(name.to_string(), context),
            Expression::UnaryOp(op, x)     => self.map_unary_op(op.clone(), &x, context),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r, context),
            Expression::Call(call, params) => self.map_call(&call, &params, context),
        }
    }

    fn map_scalar(&self, value: &ScalarT, context: &Self::Context) -> Self::Output;
    fn map_variable(&self, name: String, context: &Self::Context) -> Self::Output;
    fn map_unary_op(&self, op: UnaryOpType, x: &Expression, context: &Self::Context) -> Self::Output;
    fn map_binary_op(&self, left: &Expression, op: BinaryOpType, right: &Expression, context: &Self::Context) -> Self::Output;
    fn map_call(&self, call: &Expression, params: &Vec<Rc<Expression>>, context: &Self::Context) -> Self::Output;
}

// }}}
