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

use crate::{BinaryOpType, Expression, ScalarT, UnaryOpType};
use std::fmt;

impl fmt::Display for BinaryOpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            BinaryOpType::Sum => "Sum",
            BinaryOpType::Subtract => "Subtract",
            BinaryOpType::Product => "Product",
            BinaryOpType::Divide => "Divide",
            BinaryOpType::FloorDiv => "FloorDiv",
            BinaryOpType::Modulo => "Modulo",

            BinaryOpType::Equal => "Equal",
            BinaryOpType::NotEqual => "NotEqual",
            BinaryOpType::Greater => "Greater",
            BinaryOpType::GreaterEqual => "GreaterEqual",
            BinaryOpType::Less => "Less",
            BinaryOpType::LessEqual => "LessEqual",

            BinaryOpType::BitwiseOr => "BitwiseOr",
            BinaryOpType::BitwiseXor => "BitwiseXor",
            BinaryOpType::BitwiseAnd => "BitwiseAnd",

            BinaryOpType::LogicalAnd => "LogicalAnd",
            BinaryOpType::LogicalOr => "LogicalOr",

            BinaryOpType::LeftShift => "LeftShift",
            BinaryOpType::RightShift => "RightShift",

            BinaryOpType::Exponent => "Exponent",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for UnaryOpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            UnaryOpType::LogicalNot => "LogicalNot",
            UnaryOpType::BitwiseNot => "BitwiseNot",
            UnaryOpType::Minus => "Minus",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for ScalarT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            ScalarT::U8(x) => x.to_string(),
            ScalarT::U16(x) => x.to_string(),
            ScalarT::U32(x) => x.to_string(),
            ScalarT::U64(x) => x.to_string(),

            ScalarT::I8(x) => x.to_string(),
            ScalarT::I16(x) => x.to_string(),
            ScalarT::I32(x) => x.to_string(),
            ScalarT::I64(x) => x.to_string(),

            ScalarT::F32(x) => x.to_string(),
            ScalarT::F64(x) => x.to_string(),
        };
        write!(f, "{}", val)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Scalar(s) => write!(f, "{}", s),
            Expression::Variable(name) => write!(f, "Variable(\"{}\")", name),
            Expression::UnaryOp(op, x) => write!(f, "{}{}", op, x),
            Expression::BinaryOp(l, op, r) => write!(f, "{}({}, {})", op, l, r),
            Expression::Call(call, params) => {
                let mut param_str = format!("");
                // TODO: Make it functional. Couldn't find a neater way using fold.
                for (iparam, param) in params.iter().enumerate() {
                    param_str = if iparam == 0 {
                        format!("{}", param)
                    } else {
                        format!("{}, {}", param_str, param)
                    };
                }

                write!(f, "Call({},[{}])", call, param_str)
            }
            Expression::Subscript(aggregate, indices) => {
                let mut idx_str = format!("");
                // TODO: Make it functional. Couldn't find a neater way using fold.
                for (i_idx, idx) in indices.iter().enumerate() {
                    idx_str = if i_idx == 0 {
                        format!("{}", idx)
                    } else {
                        format!("{}, {}", idx_str, idx)
                    };
                }

                write!(f, "Subscript({},[{}])", aggregate, idx_str)
            }
        }
    }
}

impl fmt::Debug for ScalarT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            ScalarT::U8(x) => format!("U8({})", x),
            ScalarT::U16(x) => format!("U16({})", x),
            ScalarT::U32(x) => format!("U32({})", x),
            ScalarT::U64(x) => format!("U64({})", x),

            ScalarT::I8(x) => format!("I8({})", x),
            ScalarT::I16(x) => format!("I16({})", x),
            ScalarT::I32(x) => format!("I32({})", x),
            ScalarT::I64(x) => format!("I64({})", x),

            ScalarT::F32(x) => format!("F32({})", x),
            ScalarT::F64(x) => format!("F64({})", x),
        };
        write!(f, "{}", val)
    }
}
