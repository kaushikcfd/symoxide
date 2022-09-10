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

/// A numeric literal that wraps numeric literals along with their data-type
/// information.
#[derive(Copy, Clone, PartialEq)]
pub enum LiteralT {
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

/// The core expression type. The different arms of thie enum describe the
/// expression.
#[derive(Clone)]
pub enum Expression {
    /// A scalar value. Also see ['LiteralT'] for the types of scalars
    /// supported.
    Scalar(LiteralT),
    /// A variable is a symbol with an identifier in an expression. For ex. in
    /// the expression `x + y`, `x` and `y` are the variables over which an
    /// addition is performed.
    Variable(String),
    /// A unary operation on an expression. See [`UnaryOpType`] for the
    /// supported operations.
    UnaryOp(UnaryOpType, Rc<Expression>),
    /// `BinaryOp(left_operanad, op_type, right_operand)`. See [`BinaryOpType`]
    /// for the supported operations.
    BinaryOp(Rc<Expression>, BinaryOpType, Rc<Expression>),
    /// `Call(fn, args)` represents invoking the expression `fn` with the
    /// arguments `args`.
    Call(Rc<Expression>, Vec<Rc<Expression>>),
    /// `Subcript(expr, indices)` represents indexing the expression `expr` with
    /// the indices `indices`.
    Subscript(Rc<Expression>, Vec<Rc<Expression>>),
    /// `If(cond, then, else)` represents a ternary operation (AKA if-then-else)
    /// expression.
    If(Rc<Expression>, Rc<Expression>, Rc<Expression>),
}

/// Binary Operation types. Semantics of these types are purposefully kept
/// un-defined, for example Python and C disagree on their `Modulo` semantics
/// for negative denominators. And so, it is upto the downstream user to lower
/// this expression as they seem fit to.
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

/// Unary Operation types.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UnaryOpType {
    LogicalNot,
    BitwiseNot,
    Minus,
}

// }}}
