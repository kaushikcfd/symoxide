// Copyright (c) 2022 Kaushik Kulkarni
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use crate::{Expression, BinaryOpType, UnaryOpType, ScalarT};
use std::rc::Rc;


// {{{ IdentityMapper

pub trait IdentityMapper {
    fn visit(&self, expr: &Expression) -> Rc<Expression> {
        match expr {
            Expression::Variable(name)     => self.map_variable(name.to_string()),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r),
            Expression::UnaryOp(op, x)     => self.map_unary_op(op.clone(), &x),
            Expression::Scalar(s)          => self.map_scalar(&s),
        }
    }

    fn map_variable(&self, name: String) -> Rc<Expression> {
        Rc::new(Expression::Variable(name))
    }

    fn map_binary_op(&self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>) -> Rc<Expression> {
        Rc::new(Expression::BinaryOp(self.visit(left), op, self.visit(right)))
    }

    fn map_unary_op(&self, op: UnaryOpType, x: &Rc<Expression>) -> Rc<Expression> {
        Rc::new(Expression::UnaryOp(op, self.visit(x)))
    }

    fn map_scalar(&self, value: &ScalarT) -> Rc<Expression> {
        Rc::new(Expression::Scalar(value.clone()))
    }
}

// }}}


// {{{ IdentityMapperWithContext

pub trait IdentityMapperWithContext {
    type Context;

    fn visit(&self, expr: &Expression, context: &Self::Context) -> Rc<Expression> {
        match expr {
            Expression::Variable(name)     => self.map_variable(name.to_string(), context),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r, context),
            Expression::UnaryOp(op, x)     => self.map_unary_op(op.clone(), &x, context),
            Expression::Scalar(s)          => self.map_scalar(&s, context),
        }
    }

    fn map_variable(&self, name: String, _context: &Self::Context) -> Rc<Expression> {
        Rc::new(Expression::Variable(name))
    }

    fn map_binary_op(&self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>, context: &Self::Context) -> Rc<Expression> {
        Rc::new(Expression::BinaryOp(self.visit(left, context), op, self.visit(right, context)))
    }

    fn map_unary_op(&self, op: UnaryOpType, x: &Rc<Expression>, context: &Self::Context) -> Rc<Expression> {
        Rc::new(Expression::UnaryOp(op, self.visit(x, context)))
    }


    fn map_scalar(&self, value: &ScalarT, _context: &Self::Context) -> Rc<Expression> {
        Rc::new(Expression::Scalar(value.clone()))
    }
}

// }}}
