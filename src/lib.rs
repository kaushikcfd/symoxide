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

pub mod builders;
pub mod display;
pub mod macros;
pub mod mapper_impls;
pub mod mappers;
pub mod operations;
pub mod parse;
pub mod primitives;
mod utils;

pub use builders::var;
pub use macro_defs::{scalar, variables};
pub use mapper_impls::deduplicator::deduplicate_nodes;
pub use mapper_impls::dependency::get_dependencies;
pub use mapper_impls::equality::are_structurally_equal;
pub use mapper_impls::hasher::get_hasher;
pub use mapper_impls::node_counter::get_num_nodes;
pub use operations::{add, div, mul};
pub use primitives::{BinaryOpType, Expression, ScalarT, UnaryOpType};
