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


use crate::primitives::{Expression, Variable, BinaryOp};

// {{{ WalkMapper

pub trait WalkMappable: Expression {
    fn accept<T: WalkMapper>(&self, mapper: &T);
}

impl WalkMappable for Variable{
    fn accept<T: WalkMapper>(&self, mapper: &T) {
        mapper.map_variable(self)
    }
}

impl<T1: WalkMappable, T2: WalkMappable> WalkMappable for BinaryOp<T1, T2> {
    fn accept<T: WalkMapper>(&self, mapper: &T) {
        mapper.map_binary_op(self)
    }
}


pub trait WalkMapper: Sized{

    fn map_variable(&self, _expr: &Variable) {
    }

    fn map_binary_op<T1: WalkMappable, T2: WalkMappable>(
        &self, expr: &BinaryOp<T1, T2>) {
        expr.l.accept(self);
        expr.r.accept(self);
    }
}

// }}}



// {{{ WalkMapperWithContext

pub trait WalkMappableWithContext: Expression {
    fn accept<T: WalkMapperWithContext>(&self, mapper: &T, context: &T::Context);
}

impl WalkMappableWithContext for Variable{
    fn accept<T: WalkMapperWithContext>(&self, mapper: &T, context: &T::Context) {
        mapper.map_variable(self, &context)
    }
}

impl<T1: WalkMappableWithContext, T2: WalkMappableWithContext> WalkMappableWithContext for BinaryOp<T1, T2> {
    fn accept<T: WalkMapperWithContext>(&self, mapper: &T, context: &T::Context) {
        mapper.map_binary_op(self, &context)
    }
}


pub trait WalkMapperWithContext: Sized{
    type Context;

    fn map_variable(&self, _expr: &Variable, _context: &Self::Context) {
    }

    fn map_binary_op<T1: WalkMappableWithContext, T2: WalkMappableWithContext>(
            &self, expr: &BinaryOp<T1, T2>, context: &Self::Context
    ) {
        expr.l.accept(self, context);
        expr.r.accept(self, context);
    }
}

// }}}



// {{{ MutWalkMapper

pub trait MutWalkMappable: Expression {
    fn accept<T: MutWalkMapper>(&self, mapper: &mut T);
}

impl MutWalkMappable for Variable{
    fn accept<T: MutWalkMapper>(&self, mapper: &mut T) {
        mapper.map_variable(self)
    }
}

impl<T1: MutWalkMappable, T2: MutWalkMappable> MutWalkMappable for BinaryOp<T1, T2> {
    fn accept<T: MutWalkMapper>(&self, mapper: &mut T) {
        mapper.map_binary_op(self)
    }
}


pub trait MutWalkMapper: Sized{

    fn map_variable(&mut self, _expr: &Variable) {
    }

    fn map_binary_op<T1: MutWalkMappable, T2: MutWalkMappable>(
        &mut self, expr: &BinaryOp<T1, T2>) {
        expr.l.accept(self);
        expr.r.accept(self);
    }
}

// }}}
