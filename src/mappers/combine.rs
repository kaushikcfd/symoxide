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


use crate::{Expression, BinaryOpType, ScalarT};


// {{{ CombineMapper

pub trait CombineMapper: Sized{
    type Output;

    fn combine(&self, values: &[Self::Output]) -> Self::Output;

    fn visit(&self, expr: &Expression) -> Self::Output {
        match expr {
            Expression::Variable(name) => self.map_variable(name.to_string()),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r),
            Expression::Scalar(s)          => self.map_scalar(&s),
        }
    }

    fn map_variable(&self, name: String) -> Self::Output;
    fn map_scalar(&self, value: &ScalarT) -> Self::Output;

    fn map_binary_op(&self, left: &Expression, _op: BinaryOpType, right: &Expression) -> Self::Output {
        self.combine(&[self.visit(left), self.visit(right)])
    }
}

// }}}



// {{{ CombineMapperWithContext

pub trait CombineMapperWithContext: Sized{
    type Output;
    type Context;

    fn combine(&self, values: &[Self::Output]) -> Self::Output;

    fn visit(&self, expr: &Expression, context: &Self::Context) -> Self::Output {
        match expr {
            Expression::Variable(name)     => self.map_variable(name.to_string(), context),
            Expression::BinaryOp(l, op, r) => self.map_binary_op(&l, op.clone(), &r, context),
            Expression::Scalar(s)          => self.map_scalar(&s, context),
        }
    }

    fn map_variable(&self, name: String, context: &Self::Context) -> Self::Output;
    fn map_scalar(&self, value: &ScalarT, context: &Self::Context) -> Self::Output;

    fn map_binary_op(
        &self, left: &Expression, _op: BinaryOpType, right: &Expression, context: &Self::Context) -> Self::Output {
        self.combine(&[self.visit(left, context), self.visit(right, context)])
    }
}

// }}}
