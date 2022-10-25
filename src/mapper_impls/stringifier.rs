use crate::mappers::fold::FoldMapperWithContext;
use crate::mappers::CachedMapper;
use crate::primitives::{BinaryOpType, Expression, LiteralT, SmallVecExprT, UnaryOpType};
use crate::utils::ExpressionRawPointer;
use crate::CachedMapper;
use hashbrown::HashMap;
use std::fmt;
use std::rc::Rc;

const PREC_IF: u8 = 0;
const PREC_LOR: u8 = 1;
const PREC_LAND: u8 = 2;
const PREC_LNOT: u8 = 3;
const PREC_CMP: u8 = 4;
const PREC_BOR: u8 = 5;
const PREC_BXOR: u8 = 6;
const PREC_BAND: u8 = 7;
const PREC_SHIFT: u8 = 8;
const PREC_ADD: u8 = 9;
const PREC_PROD: u8 = 10;
const PREC_UNARY: u8 = 11;
const PREC_EXP: u8 = 12;
const PREC_ATOM: u8 = 13;

#[derive(CachedMapper)]
pub struct Stringifier {
    cache: HashMap<(ExpressionRawPointer, u8), String>,
}

fn guard_with_paren(my_str: String, my_prec: u8, outer_prec: &u8) -> String {
    if *outer_prec > my_prec {
        format!("({})", my_str)
    } else {
        my_str
    }
}

impl FoldMapperWithContext for Stringifier {
    type Context = u8;
    type Output = String;
    type CacheKey = (ExpressionRawPointer, u8);

    fn get_cache_key(&self, expr: &Rc<Expression>, outer_prec: &Self::Context) -> Self::CacheKey {
        (ExpressionRawPointer(expr.clone()), *outer_prec)
    }

    fn map_scalar(&mut self, value: &LiteralT, _outer_prec: &Self::Context) -> Self::Output {
        format!("{}", value)
    }
    fn map_variable(&mut self, name: String, _outer_prec: &Self::Context) -> Self::Output {
        format!("{}", name)
    }
    fn map_unary_op(&mut self, op: UnaryOpType, x: &Rc<Expression>, outer_prec: &Self::Context)
                    -> Self::Output {
        let (op_str, my_prec) = match op {
            UnaryOpType::LogicalNot => ("not ", PREC_LNOT),
            UnaryOpType::BitwiseNot => ("~", PREC_UNARY),
            UnaryOpType::Minus => ("-", PREC_UNARY),
        };

        guard_with_paren(format!("{}{}", op_str, self.visit(x, &my_prec)),
                         my_prec,
                         outer_prec)
    }
    fn map_binary_op(&mut self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>,
                     outer_prec: &Self::Context)
                     -> Self::Output {
        let op_str = match op {
            BinaryOpType::Sum => "+",
            BinaryOpType::Subtract => "-",
            BinaryOpType::Product => "*",
            BinaryOpType::Divide => "/",
            BinaryOpType::FloorDiv => "//",
            BinaryOpType::Modulo => "%",

            BinaryOpType::Equal => "==",
            BinaryOpType::NotEqual => "!=",
            BinaryOpType::Greater => ">",
            BinaryOpType::GreaterEqual => ">=",
            BinaryOpType::Less => "<",
            BinaryOpType::LessEqual => "<=",

            BinaryOpType::BitwiseOr => "|",
            BinaryOpType::BitwiseXor => "^",
            BinaryOpType::BitwiseAnd => "&",

            BinaryOpType::LogicalAnd => "and",
            BinaryOpType::LogicalOr => "or",

            BinaryOpType::LeftShift => "<<",
            BinaryOpType::RightShift => ">>",

            BinaryOpType::Exponent => "**",
        };

        let my_prec = match op {
            BinaryOpType::Sum | BinaryOpType::Subtract => PREC_ADD,
            BinaryOpType::Product
            | BinaryOpType::Divide
            | BinaryOpType::FloorDiv
            | BinaryOpType::Modulo => PREC_PROD,

            BinaryOpType::Equal
            | BinaryOpType::NotEqual
            | BinaryOpType::Greater
            | BinaryOpType::GreaterEqual
            | BinaryOpType::Less
            | BinaryOpType::LessEqual => PREC_CMP,

            BinaryOpType::BitwiseOr => PREC_BOR,
            BinaryOpType::BitwiseXor => PREC_BXOR,
            BinaryOpType::BitwiseAnd => PREC_BAND,

            BinaryOpType::LogicalAnd => PREC_LAND,
            BinaryOpType::LogicalOr => PREC_LOR,

            BinaryOpType::LeftShift | BinaryOpType::RightShift => PREC_SHIFT,

            BinaryOpType::Exponent => PREC_EXP,
        };

        let my_str = match op {
            // right-to-left associative
            BinaryOpType::Exponent => format!("{} {} {}",
                                              self.visit(left, &(my_prec + 1)),
                                              op_str,
                                              self.visit(right, &my_prec)),
            // left-to-right associative
            _ => format!("{} {} {}",
                         self.visit(left, &my_prec),
                         op_str,
                         self.visit(right, &(my_prec + 1))),
        };
        guard_with_paren(my_str, my_prec, outer_prec)
    }

    fn map_call(&mut self, call: &Rc<Expression>, params: &SmallVecExprT,
                _outer_prec: &Self::Context)
                -> Self::Output {
        let rec_str: Vec<String> = params.iter().map(|x| self.visit(x, &PREC_ATOM)).collect();
        format!("{}({})", self.visit(call, &PREC_ATOM), rec_str.join(", "))
    }
    fn map_subscript(&mut self, agg: &Rc<Expression>, indices: &SmallVecExprT,
                     _outer_prec: &Self::Context)
                     -> Self::Output {
        let rec_str: Vec<String> = indices.iter().map(|x| self.visit(x, &PREC_ATOM)).collect();
        format!("{}[{}]", self.visit(agg, &PREC_ATOM), rec_str.join(", "))
    }
    fn map_if(&mut self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>,
              outer_prec: &Self::Context)
              -> Self::Output {
        // FIXME: This might emit unnecessary parens...
        let my_prec = PREC_IF;
        let inner_prec = &(PREC_IF + 1);
        let my_str = format!("{} if {} else {}",
                             self.visit(then, inner_prec),
                             self.visit(cond, inner_prec),
                             self.visit(else_, inner_prec));
        guard_with_paren(my_str, my_prec, outer_prec)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut mapper = Stringifier { cache: HashMap::new() };
        write!(f, "{}", mapper.visit(&Rc::new(self.clone()), &0))
    }
}
