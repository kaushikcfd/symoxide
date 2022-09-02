// Copyright (c) 2022 Kaushik Kulkarni
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::primitives::Expression;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// {{{ ExpressionRawPointer

// TODO: Probably use by_address crate for this?

/// Wrap a reference of an expression as a raw pointer
/// to get trivialize comparison and hashing. Tries to
/// reproduce the effect of `id(x)` from Python world.
pub struct ExpressionRawPointer(pub Rc<Expression>);

impl ExpressionRawPointer {
    fn pointer(&self) -> *const Expression {
        let expr: &Expression = &self.0;
        expr as *const Expression
    }
}

impl PartialEq for ExpressionRawPointer {
    fn eq(&self, other: &Self) -> bool {
        self.pointer() == other.pointer()
    }
}

impl Eq for ExpressionRawPointer {}

impl Hash for ExpressionRawPointer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer().hash(state)
    }
}

// }}}
