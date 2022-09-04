use crate::mappers::fold::FoldMapperWithContext;
use crate::primitives::{BinaryOpType, Expression, ScalarT, UnaryOpType};
use std::fmt;
use std::rc::Rc;

// TODO: Use Cached Mapper here?

pub struct Reprifier {
    truncation_level: u32,
}

impl FoldMapperWithContext for Reprifier {
    type Context = u32;
    type Output = String;

    fn map_scalar(&self, value: &ScalarT, level: &Self::Context) -> Self::Output {
        if *level < self.truncation_level {
            format!("{:?}", value)
        } else {
            format!("(...)")
        }
    }
    fn map_variable(&self, name: String, level: &Self::Context) -> Self::Output {
        if *level < self.truncation_level {
            format!("Variable(\"{}\"))", name)
        } else {
            format!("(...)")
        }
    }
    fn map_unary_op(&self, op: UnaryOpType, x: &Expression, level: &Self::Context) -> Self::Output {
        if *level < self.truncation_level {
            let new_level: u32 = level + 1;
            format!("UnaryOp({}, {}))", op, self.visit(x, &new_level))
        } else {
            format!("(...)")
        }
    }
    fn map_binary_op(&self, left: &Expression, op: BinaryOpType, right: &Expression,
                     level: &Self::Context)
                     -> Self::Output {
        if *level < self.truncation_level {
            let new_level: u32 = level + 1;
            format!("BinaryOp({}, {}, {}))",
                    self.visit(left, &new_level),
                    op,
                    self.visit(right, &new_level))
        } else {
            format!("(...)")
        }
    }
    fn map_call(&self, call: &Expression, params: &Vec<Rc<Expression>>, level: &Self::Context)
                -> Self::Output {
        if *level < self.truncation_level {
            let new_level: u32 = level + 1;
            let mut param_str = format!("");
            // TODO: Make it functional. Couldn't find a neater way using fold.
            for (iparam, param) in params.iter().enumerate() {
                param_str = if iparam == 0 {
                    format!("{}", param)
                } else {
                    format!("{}, {}", param_str, self.visit(param, &new_level))
                };
            }

            format!("Call({}, [{}]))", self.visit(call, &new_level), param_str)
        } else {
            format!("(...)")
        }
    }
    fn map_subscript(&self, agg: &Expression, indices: &Vec<Rc<Expression>>,
                     level: &Self::Context)
                     -> Self::Output {
        if *level < self.truncation_level {
            let new_level: u32 = level + 1;
            let mut indices_str = format!("");
            // TODO: Make it functional. Couldn't find a neater way using fold.
            for (i_idx, idx) in indices.iter().enumerate() {
                indices_str = if i_idx == 0 {
                    format!("{}", idx)
                } else {
                    format!("{}, {}", indices_str, self.visit(idx, &new_level))
                };
            }

            format!("Subscript({}, [{}]))",
                    self.visit(agg, &new_level),
                    indices_str)
        } else {
            format!("(...)")
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mapper = Reprifier { truncation_level: 3 };
        let start_level = 0;
        write!(f, "{}", mapper.visit(&self, &start_level))
    }
}
