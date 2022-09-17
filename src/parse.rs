use crate::Expression;
use lalrpop_util::lalrpop_mod;
use lazy_static::lazy_static;
use std::rc::Rc;
use std::string::ToString;

lalrpop_mod!(py_parser, "/grammars/parse_py_flavor.rs");

lazy_static! {
    static ref PY_PARSER: py_parser::ExprParser = py_parser::ExprParser::new();
}

pub fn parse_expr<T: ToString>(input: T) -> Rc<Expression> {
    PY_PARSER.parse(input.to_string().as_str()).unwrap()
}

// vim: fdm=marker
