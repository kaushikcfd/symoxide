use crate::primitives::{Variable, Sum, Expression};


// {{{ FoldMapper

pub trait Foldable<MapperT: FoldMapper>: Expression{
    fn accept(&self, mapper: &MapperT) -> MapperT::Output;
}

impl<MapperT: FoldMapper> Foldable<MapperT> for Variable {
    fn accept(&self, mapper: &MapperT) -> MapperT::Output {
        mapper.map_variable(self)
    }
}

impl<T1: Foldable<MapperT>, T2: Foldable<MapperT>, MapperT: FoldMapper> Foldable<MapperT> for Sum<T1, T2> {
    fn accept(&self, mapper: &MapperT) -> MapperT::Output {
        mapper.map_sum(self)
    }
}


pub trait FoldMapper: Sized {
    type Output;

    fn map_variable(&self, expr: &Variable) -> Self::Output;
    fn map_sum<T1: Foldable<Self>, T2: Foldable<Self>>(&self, expr: &Sum<T1, T2>) -> Self::Output;
}

// }}}


// {{{ FoldMapperWithContext

pub trait FoldableWithContext: Expression{
    fn accept<MapperT: FoldMapperWithContext>(&self, mapper: &MapperT, context: MapperT::Context) -> MapperT::Output;
}

impl FoldableWithContext for Variable {
    fn accept<MapperT: FoldMapperWithContext>(&self, mapper: &MapperT, context: MapperT::Context) -> MapperT::Output {
        mapper.map_variable(self, context)
    }
}

impl<T1: FoldableWithContext, T2: FoldableWithContext> FoldableWithContext for Sum<T1, T2> {
    fn accept<MapperT: FoldMapperWithContext>(&self, mapper: &MapperT, context: MapperT::Context) -> MapperT::Output {
        mapper.map_sum(self, context)
    }
}


pub trait FoldMapperWithContext {
    type Context;
    type Output;

    fn map_variable(&self, expr: &Variable, context: Self::Context) -> Self::Output;
    fn map_sum<T1: FoldableWithContext, T2: FoldableWithContext>(&self, expr: &Sum<T1, T2>, context: Self::Context) -> Self::Output;
}

// }}}
