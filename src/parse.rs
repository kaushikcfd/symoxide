use crate::Expression;
use std::rc::Rc;
use lalrpop_util::lalrpop_mod;
use lazy_static::lazy_static;

lalrpop_mod!(py_parser, "/grammars/parse_py_flavor.rs");

lazy_static! {
    static ref PY_PARSER: py_parser::ExprParser = py_parser::ExprParser::new();
}


pub fn parse_expr(input: &str) -> Rc<Expression> {
    PY_PARSER.parse(input).unwrap()
}

// vim: fdm=marker
