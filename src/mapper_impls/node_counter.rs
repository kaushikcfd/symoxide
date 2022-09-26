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

use crate::mappers::walk::WalkMapper;
use crate::mappers::CachedMapper;
use crate::primitives::Expression;
use crate::utils::ExpressionRawPointer;
use crate::CachedMapper;
use hashbrown::HashMap;
use std::rc::Rc;

#[derive(CachedMapper)]
struct NodeCounter {
    num_nodes: u32,

    cache: HashMap<ExpressionRawPointer, bool>,
}

impl WalkMapper for NodeCounter {
    fn post_walk(&mut self, _expr: &Expression) {
        self.num_nodes += 1;
    }
}

pub fn get_num_nodes(expr: &Expression) -> u32 {
    let mut node_counter = NodeCounter { num_nodes: 0,
                                         cache: HashMap::new() };
    node_counter.visit(&Rc::new(expr.clone()));
    node_counter.num_nodes
}
