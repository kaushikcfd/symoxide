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
use std::fmt;

pub trait Expression: fmt::Display {}
pub trait Scalar: fmt::Display{}

#[derive(Copy, Clone)]
pub enum BinaryOpType {
    Sum,
    Product,
    Divide,
    FloorDiv,
}

pub struct Variable {
    pub name: String,
}

pub struct BinaryOp<T1: Expression + ?Sized, T2: Expression +?Sized> {
    pub op_type: BinaryOpType,
    pub l: Rc<T1>,
    pub r: Rc<T2>,
}

// }}}


// {{{ implementing Expression traits for our primitives

impl Expression for Variable {}
impl<T1: Expression + ?Sized, T2: Expression +?Sized> Expression for BinaryOp<T1, T2> {}

// }}}


// {{{ implement traits for Rust Scalars

impl Scalar for u8 {}
impl Scalar for u16 {}
impl Scalar for u32 {}
impl Scalar for u64 {}

impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}

impl Scalar for f32 {}
impl Scalar for f64 {}

impl<ScalarT: Scalar> Expression for ScalarT {}

// }}}
