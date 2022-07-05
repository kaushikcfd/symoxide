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

use std::rc::Rc;

pub trait Expression {}

pub struct Variable {
    pub name: String,
}

pub struct Sum<T1: Expression, T2: Expression> {
    pub l: Rc<T1>,
    pub r: Rc<T2>,
}


pub struct Product<T1: Expression, T2: Expression> {
    pub l: Rc<T1>,
    pub r: Rc<T2>,
}


pub struct Divide<T1: Expression, T2: Expression> {
    pub l: Rc<T1>,
    pub r: Rc<T2>,
}

// }}}


// {{{ implementing Expression traits for our primitives

impl Expression for Variable {}
pub fn add<T1: Expression, T2: Expression>(x1: &Rc<T1>, x2: &Rc<T2>) -> Sum<T1, T2>
{
    Sum {l: Rc::clone(x1), r: Rc::clone(x2)}
}
// derive_expression!(Sum<T1: Expression, T2: Expression>);
// derive_expression!(Product<T1: Expression, T2: Expression>);
// derive_expression!(Divide<T1: Expression, T2: Expression>);

// }}}


// {{{ implement traits for Rust Scalars

impl Expression for u8 {}
impl Expression for u16 {}
impl Expression for u32 {}
impl Expression for u64 {}

impl Expression for i8 {}
impl Expression for i16 {}
impl Expression for i32 {}
impl Expression for i64 {}

impl Expression for f32 {}
impl Expression for f64 {}

// }}}

// ---- Helper creation routines
/// Instantiate a new `Variable`
///
/// # Example
/// ```
/// use expression_trees::var;
///
/// let x = var("x");
/// ```
pub fn var(x: &str) -> Rc<Variable> {
    return Rc::new(Variable {name: x.to_string()});
}
