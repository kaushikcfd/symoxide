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


use crate::primitives::{Expression, Variable, Sum};


// {{{ CombineMapper

pub trait CombineMappable: Expression {
    fn accept<T: CombineMapper>(&self, mapper: &T) -> T::Output;
}

impl CombineMappable for Variable{
    fn accept<T: CombineMapper>(&self, mapper: &T) -> T::Output {
        mapper.map_variable(self)
    }
}

impl<T1: CombineMappable, T2: CombineMappable> CombineMappable for Sum<T1, T2> {
    fn accept<T: CombineMapper>(&self, mapper: &T) -> T::Output {
        mapper.map_sum(self)
    }
}


pub trait CombineMapper: Sized{
    type Output;

    fn combine(&self, values: &[Self::Output]) -> Self::Output;

    fn map_variable(&self, expr: &Variable) -> Self::Output;

    fn map_sum<T1: CombineMappable, T2: CombineMappable>(&self, expr: &Sum<T1, T2>)
                                                         -> Self::Output {
        self.combine(&[expr.l.accept(self), expr.r.accept(self)])
    }
}

// }}}



// {{{ CombineMapperWithContext

pub trait CombineMappableWithContext: Expression {
    fn accept<T: CombineMapperWithContext>(&self, mapper: &T, context: &T::Context) -> T::Output;
}

impl CombineMappableWithContext for Variable{
    fn accept<T: CombineMapperWithContext>(&self, mapper: &T, context: &T::Context) -> T::Output {
        mapper.map_variable(self, &context)
    }
}

impl<T1: CombineMappableWithContext, T2: CombineMappableWithContext> CombineMappableWithContext for Sum<T1, T2> {
    fn accept<T: CombineMapperWithContext>(&self, mapper: &T, context: &T::Context) -> T::Output {
        mapper.map_sum(self, &context)
    }
}


pub trait CombineMapperWithContext: Sized{
    type Output;
    type Context;

    fn combine(&self, values: &[Self::Output]) -> Self::Output;

    fn map_variable(&self, expr: &Variable, _context: &Self::Context) -> Self::Output;

    fn map_sum<T1: CombineMappableWithContext, T2: CombineMappableWithContext>(
            &self, expr: &Sum<T1, T2>, context: &Self::Context
    ) -> Self::Output {
        self.combine(&[expr.l.accept(self, &context), expr.r.accept(self, &context)])
    }
}

// }}}
