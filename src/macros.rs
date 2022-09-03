// Copyright (c) 2022 Kaushik Kulkarni
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

#[macro_export]
macro_rules! define_binary_op {
    ($fnName: ident, $exprName: ident) => {
        pub fn $fnName(x1: &dyn $crate::operations::ConvertibleToExpr,
                       x2: &dyn $crate::operations::ConvertibleToExpr)
                       -> std::rc::Rc<$crate::primitives::Expression> {
            std::rc::Rc::new(
                        $crate::primitives::Expression::BinaryOp(
                            x1.to_expr(),
                            $crate::primitives::BinaryOpType::$exprName,
                            x2.to_expr(),
                        )
                    )
        }
    };
}


#[macro_export]
macro_rules! rust_ty_to_scalar_type {
    (i8) => {$crate::primitives::ScalarT::I8};
    (i16) => {$crate::primitives::ScalarT::I16};
    (i32) => {$crate::primitives::ScalarT::I32};
    (i64) => {$crate::primitives::ScalarT::I64};

    (u8) => {$crate::primitives::ScalarT::U8};
    (u16) => {$crate::primitives::ScalarT::U16};
    (u32) => {$crate::primitives::ScalarT::U32};
    (u64) => {$crate::primitives::ScalarT::U64};


    (f32) => {$crate::primitives::ScalarT::F32};
    (f64) => {$crate::primitives::ScalarT::F64};
}


#[macro_export]
macro_rules! impl_scalar_to_expr {
    ($rustT:tt) => {
        impl ConvertibleToExpr for $rustT {
            fn to_expr(&self) -> Rc<Expression> {
                std::rc::Rc::new($crate::Expression::Scalar($crate::rust_ty_to_scalar_type!($rustT)(*self)))
            }
        }
    };
}
