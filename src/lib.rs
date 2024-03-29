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

//! Symoxide provides an Intermediate Representation for Scalar Expressions and
//! abstract visitors (called as mappers) for traversing and transforming these
//! expressions.
//!
//! - [`primitives`] contains the supported expression types.
//! - [`builders`] contains routines to build these expressions.
//! - [`mod@parse`] contains a parser implementation for these expressions.
//! - [`operations`] provides routines for performing common arithmetic
//!   operations on these
//! expressions.
//! - [`mappers`] provides abstract visitor for common traversal patters over
//!   scalar expressions.
//! - [`mapper_impls`] uses [`mappers`] to provide helpful analysis tools over
//!   the expressions.
//! - [`design_doc`] goes over the key design decisions that were baked into
//!   Symoxide's
//! architecture.

pub mod builders;
pub mod design_doc;
pub mod display;
pub mod macros;
pub mod mapper_impls;
pub mod mappers;
pub mod operations;
pub mod parse;
pub mod primitives;
mod utils;

pub use builders::var;
pub use mapper_impls::deduplicator::deduplicate_nodes;
pub use mapper_impls::dependency::get_dependencies;
pub use mapper_impls::equality::are_structurally_equal;
pub use mapper_impls::graphvizifier::show_dot;
pub use mapper_impls::hasher::get_hasher;
pub use mapper_impls::node_counter::get_num_nodes;
pub use parse::parse_expr as parse;
pub use primitives::{BinaryOpType, Expression, LiteralT, SmallVecExprT, UnaryOpType};
pub use symoxide_macros::{scalar, variables, CachedMapper};
pub use utils::ExpressionRawPointer;
