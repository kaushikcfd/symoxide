// Copyright (c) 2021 Kaushik Kulkarni
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

use crate::mapper_impls::equality::are_structurally_equal;
use crate::primitives::Expression;
use crate::{define_binary_op, impl_scalar_to_expr};
use std::iter::IntoIterator;
use std::rc::Rc;

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        are_structurally_equal(self, other)
    }
}

// {{{ define ConvertibleToExpr trait

pub trait ConvertibleToExpr {
    fn to_expr(&self) -> Rc<Expression>;
}

impl ConvertibleToExpr for Rc<Expression> {
    fn to_expr(&self) -> Rc<Expression> {
        self.clone()
    }
}

impl_scalar_to_expr!(i32);
impl_scalar_to_expr!(f64);

// }}}

define_binary_op!(add, Sum);
define_binary_op!(mul, Product);
define_binary_op!(div, Divide);
define_binary_op!(floor_div, FloorDiv);
define_binary_op!(modulo, Modulo);
define_binary_op!(less, Less);
define_binary_op!(less_equal, LessEqual);
define_binary_op!(greater, Greater);
define_binary_op!(greater_equal, GreaterEqual);
define_binary_op!(equal, Equal);
define_binary_op!(not_equal, GreaterEqual);
define_binary_op!(left_shift, LeftShift);
define_binary_op!(right_shift, RightShift);

/// Returns an [`Expression::Subscript`] with the expression `agg` indexed into
/// via with the indices `indices`.
pub fn index<T: IntoIterator<Item = Rc<Expression>>>(agg: Rc<Expression>, indices: T)
                                                     -> Rc<Expression> {
    let vec_indices = indices.into_iter().collect();
    Rc::new(Expression::Subscript(agg.clone(), vec_indices))
}

/// Returns an [`Expression::If`] expression.
pub fn ifthenelse(cond: Rc<Expression>, then: Rc<Expression>, else_: Rc<Expression>)
                  -> Rc<Expression> {
    Rc::new(Expression::If(cond.clone(), then.clone(), else_.clone()))
}

// vim : fdm=marker
