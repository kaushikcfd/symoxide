use crate::mappers::fold::FoldMapperWithContext;
use crate::mappers::CachedMapper;
use crate::primitives::{BinaryOpType, Expression, ScalarT, UnaryOpType};
use crate::utils::ExpressionRawPointer;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// TODO: Use Cached Mapper here?

pub struct Reprifier {
    truncation_level: u32,
    cache: HashMap<(ExpressionRawPointer, u32), String>,
}

impl CachedMapper<(ExpressionRawPointer, u32), String> for Reprifier {
    fn query_cache(&self, key: &(ExpressionRawPointer, u32)) -> Option<&String> {
        self.cache.get(key)
    }

    fn add_to_cache(&mut self, key: (ExpressionRawPointer, u32), value: String) {
        self.cache.insert(key, value);
    }
}

impl FoldMapperWithContext for Reprifier {
    type Context = u32;
    type Output = String;
    type CacheKey = (ExpressionRawPointer, u32);

    fn get_cache_key(&self, expr: &Rc<Expression>, context: &Self::Context) -> Self::CacheKey {
        (ExpressionRawPointer(expr.clone()), *context)
    }

    fn map_scalar(&mut self, value: &ScalarT, level: &Self::Context) -> Self::Output {
        if *level < self.truncation_level {
            format!("{:?}", value)
        } else {
            format!("(...)")
        }
    }
    fn map_variable(&mut self, name: String, level: &Self::Context) -> Self::Output {
        if *level < self.truncation_level {
            format!("Variable(\"{}\"))", name)
        } else {
            format!("(...)")
        }
    }
    fn map_unary_op(&mut self, op: UnaryOpType, x: &Rc<Expression>, level: &Self::Context)
                    -> Self::Output {
        if *level < self.truncation_level {
            let new_level: u32 = level + 1;
            format!("UnaryOp({}, {}))", op, self.visit(x, &new_level))
        } else {
            format!("(...)")
        }
    }
    fn map_binary_op(&mut self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>,
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
    fn map_call(&mut self, call: &Rc<Expression>, params: &Vec<Rc<Expression>>,
                level: &Self::Context)
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
    fn map_subscript(&mut self, agg: &Rc<Expression>, indices: &Vec<Rc<Expression>>,
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
    fn map_if(&mut self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>,
              level: &Self::Context)
              -> Self::Output {
        if *level < self.truncation_level {
            let new_level: u32 = level + 1;
            format!("If({}, {}, {}))",
                    self.visit(cond, &new_level),
                    self.visit(then, &new_level),
                    self.visit(else_, &new_level))
        } else {
            format!("(...)")
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut mapper = Reprifier { truncation_level: 3,
                                     cache: HashMap::new() };
        let start_level = 0;
        write!(f, "{}", mapper.visit(&Rc::new(self.clone()), &start_level))
    }
}
