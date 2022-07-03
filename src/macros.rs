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


#[macro_export]
macro_rules! _implement_binop_w_scalar {
    ($scalarT: ty, $binopName: ident, $name: ident, $( $parname: ident: $bound: path ) , *)=>{
        impl<$($parname: $bound) , *> std::ops::$binopName<$name< $($parname) , *>> for $scalarT  {
            type Output=Sum<$scalarT, $name< $($parname) , * >>;
            fn add(self, _rhs: $name< $($parname) , * >) -> Self::Output{
                $crate::primitives::Sum {l: self, r: _rhs}
            }
        }
    };
}

#[macro_export]
macro_rules! _implement_binop_w_all_scalars {
    ($binopName: ident, $name: ident, $( $parname: ident: $bound: path ) , *)=>{
        $crate::_implement_binop_w_scalar!(u8, $binopName, $name, $( $parname: $bound ) , *);
        $crate::_implement_binop_w_scalar!(u16, $binopName, $name, $( $parname: $bound ) , *);
        $crate::_implement_binop_w_scalar!(u32, $binopName, $name, $( $parname: $bound ) , *);
        $crate::_implement_binop_w_scalar!(u64, $binopName, $name, $( $parname: $bound ) , *);

        $crate::_implement_binop_w_scalar!(i8, $binopName, $name, $( $parname: $bound ) , *);
        $crate::_implement_binop_w_scalar!(i16, $binopName, $name, $( $parname: $bound ) , *);
        $crate::_implement_binop_w_scalar!(i32, $binopName, $name, $( $parname: $bound ) , *);
        $crate::_implement_binop_w_scalar!(i64, $binopName, $name, $( $parname: $bound ) , *);

        $crate::_implement_binop_w_scalar!(f32, $binopName, $name, $( $parname: $bound ) , *);
        $crate::_implement_binop_w_scalar!(f64, $binopName, $name, $( $parname: $bound ) , *);

    };
}


#[macro_export]
macro_rules! _derive_expression_internal {
    ($name: ident, $( $parname: ident: $bound: path ) , *)=>{
        impl <$( $parname: $bound) , *> $crate::primitives::Expression for $name<$($parname) , *> {}
        impl<_ExprT: $crate::primitives::Expression, $($parname: $bound) , *> std::ops::Add<_ExprT> for $name< $($parname) , * >  {
            type Output=Sum<$name< $($parname) , * >, _ExprT>;
            fn add(self, _rhs: _ExprT) -> Self::Output{
                $crate::primitives::Sum {l: self, r: _rhs}
            }
        }
        $crate::_implement_binop_w_all_scalars!(Add, $name, $( $parname: $bound ) , *);
    };
}

#[macro_export]
macro_rules! derive_expression {
    ($name: ident)=>{
        $crate::_derive_expression_internal!($name,);
    };
    ($name: ident< $par1: ident: $bound1: path, $par2: ident: $bound2: path >)=>{
        $crate::_derive_expression_internal!($name, _T1: $bound1, _T2: $bound2);
    };
}


