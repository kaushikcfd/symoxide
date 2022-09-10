use crate::mappers::fold::FoldMapper;
use crate::mappers::CachedMapper;
use crate::primitives::{BinaryOpType, Expression, LiteralT, UnaryOpType};
use crate::utils::ExpressionRawPointer;
use crate::CachedMapper;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(CachedMapper)]
pub struct HashCacher {
    cache: HashMap<ExpressionRawPointer, u64>,
}

impl HashCacher {
    pub fn get(&self, key: Rc<Expression>) -> u64 {
        let cache_key = ExpressionRawPointer(key.clone());
        match self.cache.get(&cache_key) {
            Some(x) => *x,
            None => panic!(concat!("Expression '{}' not in cache of hashed sub-expressions.",
                                   "This is most-likely an indication of using an",
                                   "invalid HashCacher instance."),
                           key),
        }
    }
}

impl FoldMapper for HashCacher {
    type Output = u64;

    fn map_scalar(&mut self, value: &LiteralT) -> Self::Output {
        let mut hasher = DefaultHasher::new();
        let cache_key = format!("{}", value);
        hasher.write(cache_key.as_bytes());
        hasher.finish()
    }
    fn map_variable(&mut self, name: String) -> Self::Output {
        let mut hasher = DefaultHasher::new();
        let cache_key = format!("Variable_{}", name);
        hasher.write(cache_key.as_bytes());
        hasher.finish()
    }
    fn map_unary_op(&mut self, op: UnaryOpType, x: &Rc<Expression>) -> Self::Output {
        let mut hasher = DefaultHasher::new();
        let op_cache_key = format!("{}", op);
        hasher.write(op_cache_key.as_bytes());
        let x_hash = self.visit(x);
        hasher.write_u64(x_hash);
        hasher.finish()
    }
    fn map_binary_op(&mut self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>)
                     -> Self::Output {
        let mut hasher = DefaultHasher::new();
        let op_cache_key = format!("{}", op);
        hasher.write(op_cache_key.as_bytes());
        let left_hash = self.visit(left);
        let right_hash = self.visit(right);
        hasher.write_u64(left_hash);
        hasher.write_u64(right_hash);
        hasher.finish()
    }
    fn map_call(&mut self, call: &Rc<Expression>, params: &Vec<Rc<Expression>>) -> Self::Output {
        let call_hash = self.visit(call);
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(call_hash);
        for param in params {
            let param_hash = self.visit(param);
            hasher.write_u64(param_hash);
        }
        hasher.finish()
    }
    fn map_subscript(&mut self, agg: &Rc<Expression>, indices: &Vec<Rc<Expression>>)
                     -> Self::Output {
        let mut hasher = DefaultHasher::new();
        let agg_hash: u64 = self.visit(agg);
        hasher.write_u64(agg_hash);
        for idx in indices {
            let idx_hash = self.visit(idx);
            hasher.write_u64(idx_hash);
        }
        hasher.finish()
    }
    fn map_if(&mut self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>)
              -> Self::Output {
        let mut hasher = DefaultHasher::new();
        let cond_hash = self.visit(cond);
        let then_hash = self.visit(then);
        let else_hash = self.visit(else_);
        hasher.write_u64(cond_hash);
        hasher.write_u64(then_hash);
        hasher.write_u64(else_hash);
        hasher.finish()
    }
}

pub fn get_hasher(expr: Rc<Expression>) -> HashCacher {
    let mut hash_cacher = HashCacher { cache: HashMap::new() };
    hash_cacher.visit(&expr.clone());
    hash_cacher
}
