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
use crate::primitives::{BinaryOpType, Expression, LiteralT, SmallVecExprT, UnaryOpType};
use crate::utils::ExpressionRawPointer;
use std::rc::Rc;

// {{{ WalkMapper

pub trait UncachedWalkMapper {
    fn should_walk(&self, _expr: &Expression) -> bool {
        true
    }

    fn post_walk(&self, _expr: &Expression) {}

    fn visit(&self, expr: &Expression) {
        if self.should_walk(expr) {
            match expr {
                Expression::Scalar(s) => self.map_scalar(&s),
                Expression::Variable(name) => self.map_variable(name.to_string()),
                Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), &x),
                Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r),
                Expression::Call(call, params) => self.map_call(&call, &params),
                Expression::Subscript(agg, indices) => self.map_subscript(&agg, &indices),
                Expression::If(cond, then, else_) => self.map_if(&cond, &then, &else_),
            };
            self.post_walk(expr);
        }
    }

    fn map_scalar(&self, _value: &LiteralT) {}

    fn map_variable(&self, _name: String) {}

    fn map_unary_op(&self, _op: UnaryOpType, x: &Expression) {
        self.visit(x);
    }

    fn map_binary_op(&self, left: &Expression, _op: BinaryOpType, right: &Expression) {
        self.visit(left);
        self.visit(right);
    }

    fn map_call(&self, call: &Expression, params: &SmallVecExprT) {
        self.visit(call);
        for param in params {
            self.visit(param);
        }
    }

    fn map_subscript(&self, agg: &Expression, indices: &SmallVecExprT) {
        self.visit(agg);
        for idx in indices {
            self.visit(idx);
        }
    }

    fn map_if(&self, cond: &Expression, then: &Expression, else_: &Expression) {
        self.visit(cond);
        self.visit(then);
        self.visit(else_);
    }
}

// }}}

// {{{ WalkMapperWithContext

pub trait WalkMapperWithContext {
    type Context;

    fn should_walk(&self, _expr: &Expression, _context: &Self::Context) -> bool {
        true
    }

    fn post_walk(&self, _expr: &Expression, _context: &Self::Context) {}

    fn visit(&self, expr: &Expression, context: &Self::Context) {
        if self.should_walk(expr, context) {
            match expr {
                Expression::Scalar(s) => self.map_scalar(&s, context),
                Expression::Variable(name) => self.map_variable(name.to_string(), context),
                Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), &x, context),
                Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r, context),
                Expression::Call(call, params) => self.map_call(&call, &params, context),
                Expression::Subscript(agg, indices) => self.map_subscript(&agg, &indices, context),
                Expression::If(cond, then, else_) => self.map_if(&cond, &then, &else_, context),
            };
            self.post_walk(expr, context);
        }
    }

    fn map_scalar(&self, _value: &LiteralT, _context: &Self::Context) {}

    fn map_variable(&self, _name: String, _context: &Self::Context) {}

    fn map_unary_op(&self, _op: UnaryOpType, x: &Expression, context: &Self::Context) {
        self.visit(x, context);
    }

    fn map_binary_op(&self, left: &Expression, _op: BinaryOpType, right: &Expression,
                     context: &Self::Context) {
        self.visit(left, context);
        self.visit(right, context);
    }

    fn map_call(&self, call: &Expression, params: &SmallVecExprT, context: &Self::Context) {
        self.visit(call, context);
        for param in params {
            self.visit(param, context);
        }
    }

    fn map_subscript(&self, agg: &Expression, indices: &SmallVecExprT, context: &Self::Context) {
        self.visit(agg, context);
        for idx in indices {
            self.visit(idx, context);
        }
    }

    fn map_if(&self, cond: &Expression, then: &Expression, else_: &Expression,
              context: &Self::Context) {
        self.visit(cond, context);
        self.visit(then, context);
        self.visit(else_, context);
    }
}

// }}}

// {{{ WalkMapper

pub trait WalkMapper: CachedMapper<ExpressionRawPointer, bool> {
    fn should_walk(&self, _expr: &Expression) -> bool {
        true
    }

    fn post_walk(&mut self, _expr: &Expression) {}

    fn visit(&mut self, expr: &Rc<Expression>) {
        let cache_key = ExpressionRawPointer(expr.clone());

        match self.query_cache(&cache_key) {
            Some(true) => {}
            None => {
                if self.should_walk(&expr) {
                    match &*expr.clone() {
                        Expression::Scalar(s) => self.map_scalar(&s),
                        Expression::Variable(name) => self.map_variable(name.to_string()),
                        Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), x),
                        Expression::BinaryOp(l, op, r) => self.map_binary_op(l, op.clone(), r),
                        Expression::Call(call, params) => self.map_call(call, &params),
                        Expression::Subscript(agg, indices) => self.map_subscript(agg, &indices),
                        Expression::If(cond, then, else_) => self.map_if(cond, then, else_),
                    };
                    self.post_walk(&expr);
                };
                self.add_to_cache(cache_key, true);
            }
            _ => unreachable!(),
        }
    }

    fn map_scalar(&mut self, _value: &LiteralT) {}

    fn map_variable(&mut self, _name: String) {}

    fn map_unary_op(&mut self, _op: UnaryOpType, x: &Rc<Expression>) {
        self.visit(x);
    }

    fn map_binary_op(&mut self, left: &Rc<Expression>, _op: BinaryOpType, right: &Rc<Expression>) {
        self.visit(left);
        self.visit(right);
    }

    fn map_call(&mut self, call: &Rc<Expression>, params: &SmallVecExprT) {
        self.visit(call);
        for param in params {
            self.visit(param);
        }
    }

    fn map_subscript(&mut self, agg: &Rc<Expression>, indices: &SmallVecExprT) {
        self.visit(agg);
        for idx in indices {
            self.visit(idx);
        }
    }

    fn map_if(&mut self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>) {
        self.visit(cond);
        self.visit(then);
        self.visit(else_);
    }
}

// }}}

// vim: fdm=marker
