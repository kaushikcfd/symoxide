// Copyright (c) 2021 Kaushik Kulkarni
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

use std::fmt;
use std::ops;
use crate::utils::ToExpression;


pub enum Expression {
    I32(i32),
    F32(f32),
    F64(f64),
    Variable(String),
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
}


impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::I32(x) => write!(f, "I32({})", x),
            Expression::F32(x) => write!(f, "F32({})", x),
            Expression::F64(x) => write!(f, "F64({})", x),
            Expression::Variable(x) => write!(f, "Variable(\"{}\")", x),
            Expression::Sum(e1, e2) => write!(f, "Sum({}, {})", e1, e2),
            Expression::Product(e1, e2) => write!(f, "Product({}, {})", e1, e2),
            _ => panic!("Not Implemented!")
        }
    }
}

// --- Operators support

// {{{ Add

impl ops::Add for Expression {
    type Output = Expression;

    fn add(self, _rhs: Expression) -> Expression {
        return Expression::Sum(Box::new(self), Box::new(_rhs));
    }
}

impl ops::Add<i32> for Expression {
    type Output = Expression;

    fn add(self, _rhs: i32) -> Expression {
        return Expression::Sum(Box::new(self), Box::new(_rhs.to_expr()));
    }
}


impl ops::Add<f32> for Expression {
    type Output = Expression;

    fn add(self, _rhs: f32) -> Expression {
        return Expression::Sum(Box::new(self), Box::new(_rhs.to_expr()));
    }
}


impl ops::Add<f64> for Expression {
    type Output = Expression;

    fn add(self, _rhs: f64) -> Expression {
        return Expression::Sum(Box::new(self), Box::new(_rhs.to_expr()));
    }
}

impl ops::Add<Expression> for i32 {
    type Output = Expression;

    fn add(self, _rhs: Expression) -> Expression {
        return Expression::Sum(Box::new(self.to_expr()), Box::new(_rhs));
    }
}

impl ops::Add<Expression> for f32 {
    type Output = Expression;

    fn add(self, _rhs: Expression) -> Expression {
        return Expression::Sum(Box::new(self.to_expr()), Box::new(_rhs));
    }
}

impl ops::Add<Expression> for f64 {
    type Output = Expression;

    fn add(self, _rhs: Expression) -> Expression {
        return Expression::Sum(Box::new(self.to_expr()), Box::new(_rhs));
    }
}

// }}}

// {{{ Mul

impl ops::Mul for Expression {
    type Output = Expression;

    fn mul(self, _rhs: Expression) -> Expression {
        return Expression::Product(Box::new(self), Box::new(_rhs));
    }
}

impl ops::Mul<f32> for Expression {
    type Output = Expression;

    fn mul(self, _rhs: f32) -> Expression {
        return Expression::Product(Box::new(self), Box::new(_rhs.to_expr()));
    }
}

impl ops::Mul<i32> for Expression {
    type Output = Expression;

    fn mul(self, _rhs: i32) -> Expression {
        return Expression::Product(Box::new(self), Box::new(_rhs.to_expr()));
    }
}

impl ops::Mul<f64> for Expression {
    type Output = Expression;

    fn mul(self, _rhs: f64) -> Expression {
        return Expression::Product(Box::new(self), Box::new(_rhs.to_expr()));
    }
}


impl ops::Mul<Expression> for i32 {
    type Output = Expression;

    fn mul(self, _rhs: Expression) -> Expression {
        return Expression::Product(Box::new(self.to_expr()), Box::new(_rhs));
    }
}

impl ops::Mul<Expression> for f32 {
    type Output = Expression;

    fn mul(self, _rhs: Expression) -> Expression {
        return Expression::Product(Box::new(self.to_expr()), Box::new(_rhs));
    }
}

impl ops::Mul<Expression> for f64 {
    type Output = Expression;

    fn mul(self, _rhs: Expression) -> Expression {
        return Expression::Product(Box::new(self.to_expr()), Box::new(_rhs));
    }
}

// }}}


// {{{ Neg

impl ops::Neg for Expression {
    type Output = Expression;

    fn neg(self) -> Expression {
        // This shouldn't override for scalars, as that's the job for downstream
        // visitors
        return -1 * self;
    }
}

// }}}

// ---- Helper creation routines
/// Instantiate a new `Expression::Variable`
///
/// # Example
/// ```
/// use expression_trees::var;
///
/// let x = var("X".to_string());
/// ```
pub fn var(x: &str) -> Expression {
    return Expression::Variable(x.to_string());
}
