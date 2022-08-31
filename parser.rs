use crate::{Expression, BinaryOpType};
use std::str::FromStr;


grammar;

pub Term: i32 = {
    <n:Num> => n,
    "(" <t:Term> ")" => t,
};

Num: i32 = <s:r"[0-9]+"> => i32::from_str(s).unwrap();

//pub Expr: Rc<Expression>  = {
//      SumExpr,
//};
//
//SumExpr: Rc<Expression> = {
//    <left_op: SumExpr> "+" <right_op: ProdExpr>   => Rc::new(Expression::BinaryOp(left_op.clone(), BinaryOpType::Sum, right_op.clone())),
//    ProdExpr,
//};
//
//ProdExpr: Rc<Expression> = {
//    <left_op: ProdExpr> "*" <right_op: AtomExpr>  => Rc::new(Expression::BinaryOp(left_op.clone(), BinaryOpType::Product, right_op.clone())),
//    AtomExpr,
//};
//
//AtomExpr: Rc<Expression> = {
//    <identifier: Identifier>                      => Rc::new(Expression::Variable(identifier)),
//};
//
//Identifier: String =  {
//    <s:r"[a-zA-Z_][a-zA-Z0-9_]*">                 => s.to_string(),
//};