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

use std::rc::Rc;

// {{{ IdentityMapper

pub trait IdentityMappable: Expression {
    fn accept<T: IdentityMapper>(&self, mapper: &T) -> Rc<dyn Expression>;
}

impl IdentityMappable for Variable{
    fn accept<T: IdentityMapper>(&self, mapper: &T) -> Rc<dyn Expression> {
        mapper.map_variable(self)
    }
}

impl<T1: IdentityMappable, T2: IdentityMappable> IdentityMappable for Sum<T1, T2> {
    fn accept<T: IdentityMapper>(&self, mapper: &T) -> Rc<dyn Expression> {
        mapper.map_sum(self)
    }
}


pub trait IdentityMapper: Sized{

    fn map_variable(&self, expr: &Variable) -> Rc<dyn Expression>{
        let result = Variable{name: expr.name.clone()};
        return Rc::new(result);
    }

    fn map_sum<T1: IdentityMappable, T2: IdentityMappable>(&self, expr: &Sum<T1, T2>) -> Rc<dyn Expression>{
        let rec_l = expr.l.accept(self);
        let rec_r = expr.r.accept(self);
        return Rc::new(Sum {l: rec_l, r: rec_r});
    }
}

// }}}



// {{{ IdentityMapperWithContext

pub trait IdentityMappableWithContext: Expression {
    fn accept<T: IdentityMapperWithContext>(&self, mapper: &T, context: &T::Context) -> Rc<dyn Expression>;
}

impl IdentityMappableWithContext for Variable{
    fn accept<T: IdentityMapperWithContext>(&self, mapper: &T, context: &T::Context) -> Rc<dyn Expression> {
        mapper.map_variable(self, &context)
    }
}

impl<T1: IdentityMappableWithContext, T2: IdentityMappableWithContext> IdentityMappableWithContext for Sum<T1, T2> {
    fn accept<T: IdentityMapperWithContext>(&self, mapper: &T, context: &T::Context) -> Rc<dyn Expression> {
        mapper.map_sum(self, &context)
    }
}


pub trait IdentityMapperWithContext: Sized{
    type Context;

    fn map_variable(&self, expr: &Variable, _context: &Self::Context) -> Rc<dyn Expression>{
        let result = Variable{name: expr.name.clone()};
        return Rc::new(result);
    }

    fn map_sum<T1: IdentityMappableWithContext, T2: IdentityMappableWithContext>(
            &self, expr: &Sum<T1, T2>, context: &Self::Context
            ) -> Rc<dyn Expression> {
        let rec_l = expr.l.accept(self, context);
        let rec_r = expr.r.accept(self, context);
        return Rc::new(Sum {l: rec_l, r: rec_r});
    }
}

// }}}
