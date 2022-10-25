// Copyright (c) 2022 Kaushik Kulkarni
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::mappers::CachedMapper;
use crate::utils::ExpressionRawPointer;
use crate::{BinaryOpType, Expression, LiteralT, SmallVecExprT, UnaryOpType};
use std::rc::Rc;

// {{{ UncachedCombineMapper

pub trait UnachedCombineMapper: Sized {
    type Output;

    fn combine(&self, values: &[Self::Output]) -> Self::Output;

    fn visit(&self, expr: &Expression) -> Self::Output {
        match expr {
            Expression::Scalar(s) => self.map_scalar(&s),
            Expression::Variable(name) => self.map_variable(name.to_string()),
            Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), &x),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r),
            Expression::Call(call, params) => self.map_call(&call, &params),
            Expression::Subscript(agg, indices) => self.map_subscript(&agg, &indices),
            Expression::If(cond, then, else_) => self.map_if(&cond, &then, &else_),
        }
    }

    fn map_scalar(&self, value: &LiteralT) -> Self::Output;
    fn map_variable(&self, name: String) -> Self::Output;

    fn map_unary_op(&self, _op: UnaryOpType, x: &Expression) -> Self::Output {
        self.visit(x)
    }

    fn map_binary_op(&self, left: &Expression, _op: BinaryOpType, right: &Expression)
                     -> Self::Output {
        self.combine(&[self.visit(left), self.visit(right)])
    }

    fn map_call(&self, call: &Expression, params: &SmallVecExprT) -> Self::Output {
        let rec_params: Vec<Self::Output> = params.iter().map(|x| self.visit(x)).collect();
        self.combine(&[self.visit(call), self.combine(&rec_params)])
    }

    fn map_subscript(&self, agg: &Expression, indices: &SmallVecExprT) -> Self::Output {
        let rec_indices: Vec<Self::Output> = indices.iter().map(|x| self.visit(x)).collect();
        self.combine(&[self.visit(agg), self.combine(&rec_indices)])
    }

    fn map_if(&self, cond: &Expression, then: &Expression, else_: &Expression) -> Self::Output {
        self.combine(&[self.visit(cond), self.visit(then), self.visit(else_)])
    }
}

// }}}

// {{{ CombineMapperWithContext

pub trait CombineMapperWithContext: Sized {
    type Output;
    type Context;

    fn combine(&self, values: &[Self::Output]) -> Self::Output;

    fn visit(&self, expr: &Expression, context: &Self::Context) -> Self::Output {
        match expr {
            Expression::Scalar(s) => self.map_scalar(&s, context),
            Expression::Variable(name) => self.map_variable(name.to_string(), context),
            Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), &x, context),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r, context),
            Expression::Call(call, params) => self.map_call(&call, &params, context),
            Expression::Subscript(agg, indices) => self.map_subscript(&agg, &indices, context),
            Expression::If(cond, then, else_) => self.map_if(&cond, &then, &else_, context),
        }
    }

    fn map_variable(&self, name: String, context: &Self::Context) -> Self::Output;
    fn map_scalar(&self, value: &LiteralT, context: &Self::Context) -> Self::Output;

    fn map_binary_op(&self, left: &Expression, _op: BinaryOpType, right: &Expression,
                     context: &Self::Context)
                     -> Self::Output {
        self.combine(&[self.visit(left, context), self.visit(right, context)])
    }

    fn map_unary_op(&self, _op: UnaryOpType, x: &Expression, context: &Self::Context)
                    -> Self::Output {
        self.visit(x, context)
    }

    fn map_call(&self, call: &Expression, params: &SmallVecExprT, context: &Self::Context)
                -> Self::Output {
        let rec_params: Vec<Self::Output> = params.iter().map(|x| self.visit(x, context)).collect();
        self.combine(&[self.visit(call, context), self.combine(&rec_params)])
    }

    fn map_subscript(&self, agg: &Expression, indices: &SmallVecExprT, context: &Self::Context)
                     -> Self::Output {
        let rec_indices: Vec<Self::Output> =
            indices.iter().map(|x| self.visit(x, context)).collect();
        self.combine(&[self.visit(agg, context), self.combine(&rec_indices)])
    }

    fn map_if(&self, cond: &Expression, then: &Expression, else_: &Expression,
              context: &Self::Context)
              -> Self::Output {
        self.combine(&[self.visit(cond, context),
                       self.visit(then, context),
                       self.visit(else_, context)])
    }
}

// }}}

// {{{ CombineMapper

pub trait CombineMapper: Sized + CachedMapper<ExpressionRawPointer, Self::Output> {
    type Output: Clone;

    fn combine(&mut self, values: &[Self::Output]) -> Self::Output;

    fn visit(&mut self, expr: &Rc<Expression>) -> Self::Output {
        let cache_key = ExpressionRawPointer(expr.clone());

        match self.query_cache(&cache_key) {
            Some(x) => x.clone(),
            None => {
                let result = match &*expr.clone() {
                    Expression::Scalar(s) => self.map_scalar(&s),
                    Expression::Variable(name) => self.map_variable(name.to_string()),
                    Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), x),
                    Expression::BinaryOp(l, op, r) => self.map_binary_op(l, op.clone(), r),
                    Expression::Call(call, params) => self.map_call(call, &params),
                    Expression::Subscript(agg, indices) => self.map_subscript(agg, &indices),
                    Expression::If(cond, then, else_) => self.map_if(cond, then, else_),
                };

                self.add_to_cache(cache_key, result.clone());
                result
            }
        }
    }

    fn map_scalar(&mut self, value: &LiteralT) -> Self::Output;
    fn map_variable(&mut self, name: String) -> Self::Output;

    fn map_unary_op(&mut self, _op: UnaryOpType, x: &Rc<Expression>) -> Self::Output {
        self.visit(x)
    }

    fn map_binary_op(&mut self, left: &Rc<Expression>, _op: BinaryOpType, right: &Rc<Expression>)
                     -> Self::Output {
        let l_rec = self.visit(left);
        let r_rec = self.visit(right);
        self.combine(&[l_rec, r_rec])
    }

    fn map_call(&mut self, call: &Rc<Expression>, params: &SmallVecExprT) -> Self::Output {
        let call_rec = self.visit(call);
        let rec_params: Vec<Self::Output> = params.iter().map(|x| self.visit(x)).collect();
        let combined_params = self.combine(&rec_params);
        self.combine(&[call_rec, combined_params])
    }

    fn map_subscript(&mut self, agg: &Rc<Expression>, indices: &SmallVecExprT) -> Self::Output {
        let agg_rec = self.visit(agg);
        let rec_indices: Vec<Self::Output> = indices.iter().map(|x| self.visit(x)).collect();
        let combined_params = self.combine(&rec_indices);
        self.combine(&[agg_rec, combined_params])
    }

    fn map_if(&mut self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>)
              -> Self::Output {
        let cond_rec = self.visit(cond);
        let then_rec = self.visit(then);
        let else_rec = self.visit(else_);
        self.combine(&[cond_rec, then_rec, else_rec])
    }
}

// }}}

// vim: fdm=marker
