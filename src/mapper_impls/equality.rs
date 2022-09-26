// Copyright 2022 Kaushik Kulkarni
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

use crate::primitives::{BinaryOpType, Expression, LiteralT, UnaryOpType};
use crate::utils::ExpressionRawPointer;
use hashbrown::HashMap;
use std::rc::Rc;

struct EqualityMapper {
    cache: HashMap<(ExpressionRawPointer, ExpressionRawPointer), bool>,
}

// {{{ EqualityMapper

impl EqualityMapper {
    fn visit(&mut self, expr1: Rc<Expression>, expr2: Rc<Expression>) -> bool {
        let cache_key = (ExpressionRawPointer(expr1.clone()), ExpressionRawPointer(expr2.clone()));
        match self.cache.get(&cache_key) {
            Some(x) => *x,
            None => {
                let result = if Rc::ptr_eq(&expr1, &expr2) {
                    true
                } else {
                    match &*(expr1.clone()) {
                        Expression::Scalar(s) => self.map_scalar(*s, expr2),
                        Expression::Variable(name) => self.map_variable(name.to_string(), expr2),
                        Expression::UnaryOp(op, x) => self.map_unary_op(*op, x.clone(), expr2),
                        Expression::BinaryOp(l, op, r) => {
                            self.map_binary_op(l.clone(), *op, r.clone(), expr2)
                        }
                        Expression::Call(call, params) => {
                            self.map_call(call.clone(), &params, expr2)
                        }
                        Expression::Subscript(agg, indices) => {
                            self.map_subscript(agg.clone(), &indices, expr2)
                        }
                        Expression::If(cond, then, else_) => {
                            self.map_if(cond.clone(), then.clone(), else_.clone(), expr2)
                        }
                    }
                };
                self.cache.insert(cache_key, result);
                result
            }
        }
    }

    fn map_scalar(&self, value: LiteralT, expr2: Rc<Expression>) -> bool {
        match *expr2 {
            Expression::Scalar(value2) => value == value2,
            _ => false,
        }
    }

    fn map_variable(&mut self, name: String, expr2: Rc<Expression>) -> bool {
        match &*expr2 {
            Expression::Variable(name2) => name == *name2,
            _ => false,
        }
    }

    fn map_unary_op(&mut self, op: UnaryOpType, x: Rc<Expression>, expr2: Rc<Expression>) -> bool {
        match &*expr2 {
            Expression::UnaryOp(op2, x2) => (op == *op2) && self.visit(x.clone(), x2.clone()),
            _ => false,
        }
    }
    fn map_binary_op(&mut self, left: Rc<Expression>, op: BinaryOpType, right: Rc<Expression>,
                     expr2: Rc<Expression>)
                     -> bool {
        match &*expr2 {
            Expression::BinaryOp(left2, op2, right2) => {
                (op == *op2)
                && self.visit(left.clone(), left2.clone())
                && self.visit(right.clone(), right2.clone())
            }
            _ => false,
        }
    }

    fn map_call(&mut self, call: Rc<Expression>, params: &Vec<Rc<Expression>>,
                expr2: Rc<Expression>)
                -> bool {
        match &*expr2 {
            Expression::Call(call2, params2) => {
                (params.len() == params2.len())
                && self.visit(call.clone(), call2.clone())
                && (params.iter()
                          .zip(params2.iter())
                          .fold(true, |acc, (par1, par2)| {
                              acc && self.visit(par1.clone(), par2.clone())
                          }))
            }
            _ => false,
        }
    }
    fn map_subscript(&mut self, agg: Rc<Expression>, indices: &Vec<Rc<Expression>>,
                     expr2: Rc<Expression>)
                     -> bool {
        match &*expr2 {
            Expression::Subscript(agg2, indices2) => {
                (indices.len() == indices2.len())
                && self.visit(agg.clone(), agg2.clone())
                && (indices.iter()
                           .zip(indices2.iter())
                           .fold(true, |acc, (idx1, idx2)| {
                               acc && self.visit(idx1.clone(), idx2.clone())
                           }))
            }
            _ => false,
        }
    }

    fn map_if(&mut self, cond: Rc<Expression>, then: Rc<Expression>, else_: Rc<Expression>,
              expr2: Rc<Expression>)
              -> bool {
        match &*expr2 {
            Expression::If(cond2, then2, else2) => {
                self.visit(cond.clone(), cond2.clone())
                && self.visit(then.clone(), then2.clone())
                && self.visit(else_.clone(), else2.clone())
            }
            _ => false,
        }
    }
}

// }}}

pub fn are_structurally_equal(expr1: &Expression, expr2: &Expression) -> bool {
    if std::ptr::eq(expr1, expr2) {
        true
    } else {
        let mut mapper = EqualityMapper { cache: HashMap::new() };
        mapper.visit(Rc::new(expr1.clone()), Rc::new(expr2.clone()))
    }
}
