use crate::Expression;
extern crate pest;
use pest::Parser;


#[derive(Parser)]
#[grammar = "expr_py_flavor.peg"]
pub struct PyExprParser;


pub fn parse_expr(input: &str) -> Expression {
    let successful_parse = PyExprParser::parse(Rule::expr, input);
    println!("{:?}", successful_parse);

    unimplemented!();
}

// vim: fdm=marker
