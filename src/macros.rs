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
macro_rules! derive_expression {
    ($name: ident)=>{
        impl Expression for $name {}
        impl<_ExprT: Expression> std::ops::Add<_ExprT> for $name {
            type Output=Sum<$name, _ExprT>;
            fn add(self, _rhs: _ExprT) -> Self::Output{
                $crate::primitives::Sum {l: self, r: _rhs}
            }
        }
    };
    // TODO: Generalize it as: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e18c5d3415162283ff3ed0480205e5e2
    ($name: ident, $bound1: path, $bound2: path)=>{
        impl<_T1: $bound1, _T2: $bound2> Expression for $name<_T1, _T2> {}
        impl<_ExprT: Expression, _T1: $bound1, _T2: $bound2> std::ops::Add<_ExprT> for $name<_T1, _T2> {
            type Output=Sum<$name<_T1, _T2>, _ExprT>;
            fn add(self, _rhs: _ExprT) -> Self::Output{
                $crate::primitives::Sum {l: self, r: _rhs}
            }
        }
    };
}
