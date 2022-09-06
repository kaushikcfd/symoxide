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
use crate::{BinaryOpType, Expression, ScalarT, UnaryOpType};
use std::rc::Rc;

// {{{ IdentityMapper

pub trait IdentityMapper: CachedMapper<ExpressionRawPointer, Rc<Expression>> {
    fn visit(&mut self, expr: Rc<Expression>) -> Rc<Expression> {
        let cache_key = ExpressionRawPointer(expr.clone());
        match self.query_cache(&cache_key) {
            Some(x) => x.clone(),
            None => {
                let result = match &*expr {
                    Expression::Scalar(s) => self.map_scalar(&s),
                    Expression::Variable(name) => self.map_variable(name.to_string()),
                    Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), &x),
                    Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r),
                    Expression::Call(call, params) => self.map_call(&call, &params),
                    Expression::Subscript(agg, indices) => self.map_subscript(&agg, &indices),
                    Expression::If(cond, then, else_) => self.map_if(&cond, &then, &else_),
                };
                self.add_to_cache(cache_key, result.clone());
                result
            }
        }
    }

    fn map_scalar(&mut self, value: &ScalarT) -> Rc<Expression> {
        Rc::new(Expression::Scalar(value.clone()))
    }

    fn map_variable(&mut self, name: String) -> Rc<Expression> {
        Rc::new(Expression::Variable(name))
    }

    fn map_unary_op(&mut self, op: UnaryOpType, x: &Rc<Expression>) -> Rc<Expression> {
        Rc::new(Expression::UnaryOp(op, self.visit(x.clone())))
    }

    fn map_binary_op(&mut self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>)
                     -> Rc<Expression> {
        Rc::new(Expression::BinaryOp(self.visit(left.clone()), op, self.visit(right.clone())))
    }

    fn map_call(&mut self, call: &Rc<Expression>, params: &Vec<Rc<Expression>>) -> Rc<Expression> {
        Rc::new(Expression::Call(self.visit(call.clone()),
                                 params.iter()
                                       .map(|param| self.visit(param.clone()))
                                       .collect()))
    }

    fn map_subscript(&mut self, agg: &Rc<Expression>, indices: &Vec<Rc<Expression>>)
                     -> Rc<Expression> {
        Rc::new(Expression::Subscript(self.visit(agg.clone()),
                                      indices.iter().map(|idx| self.visit(idx.clone())).collect()))
    }

    fn map_if(&mut self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>)
              -> Rc<Expression> {
        Rc::new(Expression::If(self.visit(cond.clone()),
                               self.visit(then.clone()),
                               self.visit(else_.clone())))
    }
}

// }}}

// {{{ UncachedIdentityMapper

pub trait UncachedIdentityMapper {
    fn visit(&self, expr: &Expression) -> Rc<Expression> {
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

    fn map_scalar(&self, value: &ScalarT) -> Rc<Expression> {
        Rc::new(Expression::Scalar(value.clone()))
    }

    fn map_variable(&self, name: String) -> Rc<Expression> {
        Rc::new(Expression::Variable(name))
    }

    fn map_unary_op(&self, op: UnaryOpType, x: &Rc<Expression>) -> Rc<Expression> {
        Rc::new(Expression::UnaryOp(op, self.visit(x)))
    }

    fn map_binary_op(&self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>)
                     -> Rc<Expression> {
        Rc::new(Expression::BinaryOp(self.visit(left), op, self.visit(right)))
    }

    fn map_call(&self, call: &Rc<Expression>, params: &Vec<Rc<Expression>>) -> Rc<Expression> {
        Rc::new(Expression::Call(self.visit(call),
                                 params.iter().map(|param| self.visit(param)).collect()))
    }

    fn map_subscript(&self, agg: &Rc<Expression>, indices: &Vec<Rc<Expression>>) -> Rc<Expression> {
        Rc::new(Expression::Subscript(self.visit(agg),
                                      indices.iter().map(|idx| self.visit(idx)).collect()))
    }

    fn map_if(&self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>)
              -> Rc<Expression> {
        Rc::new(Expression::If(self.visit(cond), self.visit(then), self.visit(else_)))
    }
}

// }}}

// {{{ IdentityMapperWithContext

pub trait IdentityMapperWithContext {
    type Context;

    fn visit(&self, expr: &Expression, context: &Self::Context) -> Rc<Expression> {
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

    fn map_scalar(&self, value: &ScalarT, _context: &Self::Context) -> Rc<Expression> {
        Rc::new(Expression::Scalar(value.clone()))
    }

    fn map_variable(&self, name: String, _context: &Self::Context) -> Rc<Expression> {
        Rc::new(Expression::Variable(name))
    }

    fn map_unary_op(&self, op: UnaryOpType, x: &Rc<Expression>, context: &Self::Context)
                    -> Rc<Expression> {
        Rc::new(Expression::UnaryOp(op, self.visit(x, context)))
    }

    fn map_binary_op(&self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>,
                     context: &Self::Context)
                     -> Rc<Expression> {
        Rc::new(Expression::BinaryOp(self.visit(left, context), op, self.visit(right, context)))
    }

    fn map_call(&self, call: &Rc<Expression>, params: &Vec<Rc<Expression>>,
                context: &Self::Context)
                -> Rc<Expression> {
        Rc::new(Expression::Call(self.visit(call, context),
                                 params.iter()
                                       .map(|param| self.visit(param, context))
                                       .collect()))
    }

    fn map_subscript(&self, agg: &Rc<Expression>, indices: &Vec<Rc<Expression>>,
                     context: &Self::Context)
                     -> Rc<Expression> {
        Rc::new(Expression::Subscript(self.visit(agg, context),
                                      indices.iter().map(|idx| self.visit(idx, context)).collect()))
    }

    fn map_if(&self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>,
              context: &Self::Context)
              -> Rc<Expression> {
        Rc::new(Expression::If(self.visit(cond, context),
                               self.visit(then, context),
                               self.visit(else_, context)))
    }
}

// }}}

// {{{ IdentityMapperWithCustomCacheKey

pub trait IdentityMapperWithCustomCacheKey: CachedMapper<Self::CacheKey, Rc<Expression>> {
    type CacheKey;

    fn get_cache_key(&self, expr: Rc<Expression>) -> Self::CacheKey;

    fn visit(&mut self, expr: Rc<Expression>) -> Rc<Expression> {
        let cache_key = self.get_cache_key(expr.clone());
        match self.query_cache(&cache_key) {
            Some(x) => x.clone(),
            None => {
                let result = match &*expr {
                    Expression::Scalar(s) => self.map_scalar(&s),
                    Expression::Variable(name) => self.map_variable(name.to_string()),
                    Expression::UnaryOp(op, x) => self.map_unary_op(op.clone(), &x),
                    Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r),
                    Expression::Call(call, params) => self.map_call(&call, &params),
                    Expression::Subscript(agg, indices) => self.map_subscript(&agg, &indices),
                    Expression::If(cond, then, else_) => self.map_if(&cond, &then, &else_),
                };
                self.add_to_cache(cache_key, result.clone());
                result
            }
        }
    }

    fn map_scalar(&mut self, value: &ScalarT) -> Rc<Expression> {
        Rc::new(Expression::Scalar(value.clone()))
    }

    fn map_variable(&mut self, name: String) -> Rc<Expression> {
        Rc::new(Expression::Variable(name))
    }

    fn map_unary_op(&mut self, op: UnaryOpType, x: &Rc<Expression>) -> Rc<Expression> {
        Rc::new(Expression::UnaryOp(op, self.visit(x.clone())))
    }

    fn map_binary_op(&mut self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>)
                     -> Rc<Expression> {
        Rc::new(Expression::BinaryOp(self.visit(left.clone()), op, self.visit(right.clone())))
    }

    fn map_call(&mut self, call: &Rc<Expression>, params: &Vec<Rc<Expression>>) -> Rc<Expression> {
        Rc::new(Expression::Call(self.visit(call.clone()),
                                 params.iter()
                                       .map(|param| self.visit(param.clone()))
                                       .collect()))
    }

    fn map_subscript(&mut self, agg: &Rc<Expression>, indices: &Vec<Rc<Expression>>)
                     -> Rc<Expression> {
        Rc::new(Expression::Subscript(self.visit(agg.clone()),
                                      indices.iter().map(|idx| self.visit(idx.clone())).collect()))
    }

    fn map_if(&mut self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>)
              -> Rc<Expression> {
        Rc::new(Expression::If(self.visit(cond.clone()),
                               self.visit(then.clone()),
                               self.visit(else_.clone())))
    }
}

// }}}
