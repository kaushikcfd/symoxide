use crate::primitives::{Variable, BinaryOp, Expression};


// {{{ FoldMapper

pub trait Foldable: Expression{
    fn accept<MapperT: FoldMapper>(&self, mapper: &MapperT) -> MapperT::Output;
}

impl Foldable for Variable {
    fn accept<MapperT: FoldMapper>(&self, mapper: &MapperT) -> MapperT::Output {
        mapper.map_variable(self)
    }
}

impl<T1: Foldable, T2: Foldable> Foldable for BinaryOp<T1, T2> {
    fn accept<MapperT: FoldMapper>(&self, mapper: &MapperT) -> MapperT::Output {
        mapper.map_binary_op(self)
    }
}


pub trait FoldMapper: Sized {
    type Output;

    fn map_variable(&self, expr: &Variable) -> Self::Output;
    fn map_binary_op<T1: Foldable, T2: Foldable>(&self, expr: &BinaryOp<T1, T2>) -> Self::Output;
}

// }}}


// {{{ FoldMapperWithContext

pub trait FoldableWithContext: Expression{
    fn accept<MapperT: FoldMapperWithContext>(&self, mapper: &MapperT, context: &MapperT::Context) -> MapperT::Output;
}

impl FoldableWithContext for Variable {
    fn accept<MapperT: FoldMapperWithContext>(&self, mapper: &MapperT, context: &MapperT::Context) -> MapperT::Output {
        mapper.map_variable(self, &context)
    }
}

impl<T1: FoldableWithContext, T2: FoldableWithContext> FoldableWithContext for BinaryOp<T1, T2> {
    fn accept<MapperT: FoldMapperWithContext>(&self, mapper: &MapperT, context: &MapperT::Context) -> MapperT::Output {
        mapper.map_binary_op(self, &context)
    }
}


pub trait FoldMapperWithContext {
    type Context;
    type Output;

    fn map_variable(&self, expr: &Variable, context: &Self::Context) -> Self::Output;
    fn map_binary_op<T1: FoldableWithContext, T2: FoldableWithContext>(&self, expr: &BinaryOp<T1, T2>, context: &Self::Context) -> Self::Output;
}

// }}}
