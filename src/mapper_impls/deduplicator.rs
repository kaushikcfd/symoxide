// Copyright (c) 2022 Kaushik Kulkarni
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::mapper_impls::hasher::{get_hasher, HashCacher};
use crate::mappers::identity::IdentityMapperWithCustomCacheKey;
use crate::mappers::CachedMapper;
use crate::{CachedMapper, Expression};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// {{{ HashedExpression

struct HashedExpression {
    expr: Rc<Expression>,
    hashval: u64,
}

impl Hash for HashedExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hashval.hash(state)
    }
}

impl PartialEq for HashedExpression {
    fn eq(&self, other: &Self) -> bool {
        self.hashval == other.hashval && self.expr == other.expr
    }
}

impl Eq for HashedExpression {}

// }}}

#[derive(CachedMapper)]
struct Deduplicator {
    hasher: HashCacher,
    cache: HashMap<HashedExpression, Rc<Expression>>,
}

impl IdentityMapperWithCustomCacheKey for Deduplicator {
    type CacheKey = HashedExpression;

    fn get_cache_key(&self, expr: Rc<Expression>) -> HashedExpression {
        HashedExpression { expr: expr.clone(),
                           hashval: self.hasher.get(expr.clone()) }
    }
}

pub fn deduplicate_nodes(expr: &Expression) -> Rc<Expression> {
    let rc_expr = Rc::new(expr.clone());
    let hasher = get_hasher(rc_expr.clone());
    let mut mapper = Deduplicator { hasher: hasher,
                                    cache: HashMap::new() };
    mapper.visit(rc_expr.clone())
}
