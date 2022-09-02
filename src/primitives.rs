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

use std::rc::Rc;

#[derive(Copy, Clone, PartialEq)]
pub enum ScalarT {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),
}

pub enum Expression {
    Scalar(ScalarT),
    Variable(String),
    UnaryOp(UnaryOpType, Rc<Expression>),
    BinaryOp(Rc<Expression>, BinaryOpType, Rc<Expression>),
    Call(Rc<Expression>, Vec<Rc<Expression>>),
    Subscript(Rc<Expression>, Vec<Rc<Expression>>),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BinaryOpType {
    Sum,
    Subtract,
    Product,
    Divide,
    FloorDiv,
    Modulo,

    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,

    LogicalAnd,
    LogicalOr,

    LeftShift,
    RightShift,

    Exponent,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UnaryOpType {
    LogicalNot,
    BitwiseNot,
    Minus,
}

// }}}
