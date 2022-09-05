use crate::mappers::CachedMapper;
use crate::primitives::{BinaryOpType, Expression, ScalarT, UnaryOpType};
use crate::utils::ExpressionRawPointer;
use std::rc::Rc;

// {{{ FoldMapper

pub trait FoldMapper: CachedMapper<ExpressionRawPointer, Self::Output> {
    type Output: Clone;

    fn visit(&mut self, expr: Rc<Expression>) -> Self::Output {
        let cache_key = ExpressionRawPointer(expr.clone());
        match self.query_cache(&cache_key) {
            Some(x) => x.clone(),
            None => {
                let result = match &*expr {
                    Expression::Scalar(s) => self.map_scalar(&s),
                    Expression::Variable(name) => self.map_variable(name.to_string()),
                    Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), x.clone()),
                    Expression::BinaryOp(l, op, r) => {
                        self.map_binary_op(l.clone(), op.clone(), r.clone())
                    }
                    Expression::Call(call, params) => self.map_call(call.clone(), &params),
                    Expression::Subscript(agg, indices) => self.map_subscript(agg.clone(), indices),
                };
                self.add_to_cache(cache_key, result.clone());
                result
            }
        }
    }

    fn map_scalar(&mut self, value: &ScalarT) -> Self::Output;
    fn map_variable(&mut self, name: String) -> Self::Output;
    fn map_unary_op(&mut self, op: UnaryOpType, x: Rc<Expression>) -> Self::Output;
    fn map_binary_op(&mut self, left: Rc<Expression>, op: BinaryOpType, right: Rc<Expression>)
                     -> Self::Output;
    fn map_call(&mut self, call: Rc<Expression>, params: &Vec<Rc<Expression>>) -> Self::Output;
    fn map_subscript(&mut self, agg: Rc<Expression>, indices: &Vec<Rc<Expression>>)
                     -> Self::Output;
}

// }}}

// {{{ UncachedFoldMapper

pub trait UncachedFoldMapper {
    type Output;

    fn visit(&self, expr: &Expression) -> Self::Output {
        match expr {
            Expression::Scalar(s) => self.map_scalar(&s),
            Expression::Variable(name) => self.map_variable(name.to_string()),
            Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), &x),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r),
            Expression::Call(call, params) => self.map_call(&call, &params),
            Expression::Subscript(agg, indices) => self.map_subscript(agg, indices),
        }
    }

    fn map_scalar(&self, value: &ScalarT) -> Self::Output;
    fn map_variable(&self, name: String) -> Self::Output;
    fn map_unary_op(&self, op: UnaryOpType, x: &Expression) -> Self::Output;
    fn map_binary_op(&self, left: &Expression, op: BinaryOpType, right: &Expression)
                     -> Self::Output;
    fn map_call(&self, call: &Expression, params: &Vec<Rc<Expression>>) -> Self::Output;
    fn map_subscript(&self, agg: &Expression, indices: &Vec<Rc<Expression>>) -> Self::Output;
}

// }}}

// {{{ FoldMapperWithContext

pub trait FoldMapperWithContext {
    type Context;
    type Output;

    fn visit(&self, expr: &Expression, context: &Self::Context) -> Self::Output {
        match expr {
            Expression::Scalar(s) => self.map_scalar(&s, context),
            Expression::Variable(name) => self.map_variable(name.to_string(), context),
            Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), &x, context),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r, context),
            Expression::Call(call, params) => self.map_call(&call, &params, context),
            Expression::Subscript(agg, indices) => self.map_subscript(&agg, &indices, context),
        }
    }

    fn map_scalar(&self, value: &ScalarT, context: &Self::Context) -> Self::Output;
    fn map_variable(&self, name: String, context: &Self::Context) -> Self::Output;
    fn map_unary_op(&self, op: UnaryOpType, x: &Expression, context: &Self::Context)
                    -> Self::Output;
    fn map_binary_op(&self, left: &Expression, op: BinaryOpType, right: &Expression,
                     context: &Self::Context)
                     -> Self::Output;
    fn map_call(&self, call: &Expression, params: &Vec<Rc<Expression>>, context: &Self::Context)
                -> Self::Output;
    fn map_subscript(&self, agg: &Expression, indices: &Vec<Rc<Expression>>,
                     context: &Self::Context)
                     -> Self::Output;
}

// }}}

// {{{ MutFoldMapperWithContext

pub trait MutFoldMapperWithContext {
    type Context;
    type Output;

    fn visit(&mut self, expr: &Expression, context: &Self::Context) -> Self::Output {
        match expr {
            Expression::Scalar(s) => self.map_scalar(&s, context),
            Expression::Variable(name) => self.map_variable(name.to_string(), context),
            Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), &x, context),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r, context),
            Expression::Call(call, params) => self.map_call(&call, &params, context),
            Expression::Subscript(agg, indices) => self.map_subscript(&agg, &indices, context),
        }
    }

    fn map_scalar(&mut self, value: &ScalarT, context: &Self::Context) -> Self::Output;
    fn map_variable(&mut self, name: String, context: &Self::Context) -> Self::Output;
    fn map_unary_op(&mut self, op: UnaryOpType, x: &Expression, context: &Self::Context)
                    -> Self::Output;
    fn map_binary_op(&mut self, left: &Expression, op: BinaryOpType, right: &Expression,
                     context: &Self::Context)
                     -> Self::Output;
    fn map_call(&mut self, call: &Expression, params: &Vec<Rc<Expression>>,
                context: &Self::Context)
                -> Self::Output;
    fn map_subscript(&mut self, agg: &Expression, indices: &Vec<Rc<Expression>>,
                     context: &Self::Context)
                     -> Self::Output;
}

// }}}
