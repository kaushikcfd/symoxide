use crate::Expression;
use std::rc::Rc;
use lalrpop_util::lalrpop_mod;

// The grammar we are parsing is as follows in a EBNF grammar
// expr = logical_or
// logical_or = logical_or OR logical_and
//            | logical_and
// logical_and = logical_and AND logical_not
//             | logical_not
// logical_not = NOT logical_not
//             | cmp_eq
// cmp_eq      = cmp_eq DOUBLE_EQ cmp_neq
//             | cmp_neq
// cmp_neq     = cmp_neq NEQ cmp_gt
//             | cmp_gt
// cmp_gt      = cmp_gt GT cm

lalrpop_mod!(parser, "/grammars/parse_py_flavor.rs");


pub fn parse_expr(input: &str) -> Rc<Expression> {
    parser::ExprParser::new().parse(input).unwrap()
}

// vim: fdm=marker
