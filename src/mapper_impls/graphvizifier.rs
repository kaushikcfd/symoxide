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

use crate::mappers::fold::FoldMapper;
use crate::mappers::CachedMapper;
use crate::primitives::{BinaryOpType, Expression, LiteralT, UnaryOpType};
use crate::utils::ExpressionRawPointer;
use crate::CachedMapper;
use pytools_rs::{
    make_unique_name_gen, show_dot as show_dot_code, ConvertibleToDotOutputT, UniqueNameGenerator,
};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(CachedMapper)]
struct Graphvizifier {
    vng: UniqueNameGenerator,
    node_descrs: Vec<String>,
    edge_descrs: Vec<String>,
    cache: HashMap<ExpressionRawPointer, String>,
}

fn pprint_binop(op: &BinaryOpType) -> &str {
    match op {
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
    }
}

fn pprint_uop(op: &UnaryOpType) -> &str {
    match op {
        UnaryOpType::LogicalNot => "not",
        UnaryOpType::BitwiseNot => "~",
        UnaryOpType::Minus => "-",
    }
}

impl FoldMapper for Graphvizifier {
    type Output = String;

    fn map_scalar(&mut self, value: &LiteralT) -> Self::Output {
        let node_name = self.vng.get("expr");
        self.node_descrs
            .push(format!("{} [label=\"{}\"]", node_name, value));
        node_name.to_string()
    }
    fn map_variable(&mut self, name: String) -> Self::Output {
        let node_name = self.vng.get("expr");
        self.node_descrs
            .push(format!("{} [label=\"{}\"]", node_name, name));
        node_name.to_string()
    }
    fn map_unary_op(&mut self, op: UnaryOpType, x: &Rc<Expression>) -> Self::Output {
        let node_name = self.vng.get("expr");
        let x_name = self.visit(x);

        self.node_descrs
            .push(format!("{} [label=\"{}\"]", node_name, pprint_uop(&op)));
        self.edge_descrs
            .push(format!("{} -> {}", x_name, node_name));
        node_name.to_string()
    }
    fn map_binary_op(&mut self, left: &Rc<Expression>, op: BinaryOpType, right: &Rc<Expression>)
                     -> Self::Output {
        let node_name = self.vng.get("expr");
        let left_node_name = self.visit(left);
        let right_node_name = self.visit(right);

        self.node_descrs
            .push(format!("{} [label=\"{}\"]", node_name, pprint_binop(&op)));
        self.edge_descrs
            .push(format!("{} -> {}", left_node_name, node_name));
        self.edge_descrs
            .push(format!("{} -> {}", right_node_name, node_name));
        node_name.to_string()
    }
    fn map_call(&mut self, call: &Rc<Expression>, params: &Vec<Rc<Expression>>) -> Self::Output {
        let node_name = self.vng.get("expr");
        let call_node_name = self.visit(call);
        let params_strs: Vec<String> = params.iter()
                                             .enumerate()
                                             .map(|(i, _)| format!("arg{}", i))
                                             .collect();
        let label = format!("Fn({})", params_strs.join(", "));

        self.node_descrs
            .push(format!("{} [label=\"{}\"]", node_name, label));
        self.edge_descrs
            .push(format!("{} -> {} [label=\"Fn\"]", call_node_name, node_name));

        for (iparam, param) in params.iter().enumerate() {
            let param_node_name = self.visit(param);
            self.edge_descrs.push(format!("{} -> {} [label=\"arg{}\"]",
                                          param_node_name, node_name, iparam));
        }
        node_name.to_string()
    }
    fn map_subscript(&mut self, agg: &Rc<Expression>, indices: &Vec<Rc<Expression>>)
                     -> Self::Output {
        let node_name = self.vng.get("expr");
        let indices_strs: Vec<String> = indices.iter()
                                               .enumerate()
                                               .map(|(i, _)| format!("i{}", i))
                                               .collect();
        let label = format!("A[{}]", indices_strs.join(", "));

        self.node_descrs
            .push(format!("{} [label=\"{}\"]", node_name, label));
        let agg_node_name = self.visit(agg);
        self.edge_descrs
            .push(format!("{} -> {} [label=\"A\"]", agg_node_name, node_name));

        for (i_idx, idx) in indices.iter().enumerate() {
            let idx_node_name = self.visit(idx);
            self.edge_descrs
                .push(format!("{} -> {} [label=\"i{}\"]", idx_node_name, node_name, i_idx));
        }

        node_name.to_string()
    }
    fn map_if(&mut self, cond: &Rc<Expression>, then: &Rc<Expression>, else_: &Rc<Expression>)
              -> Self::Output {
        let node_name = self.vng.get("expr");
        let cond_node_name = self.visit(cond);
        let then_node_name = self.visit(then);
        let else_node_name = self.visit(else_);

        self.node_descrs
            .push(format!("{} [label=\"X if Y else Z\"]", node_name));
        self.edge_descrs
            .push(format!("{} -> {}", cond_node_name, node_name));
        self.edge_descrs
            .push(format!("{} -> {}", then_node_name, node_name));
        self.edge_descrs
            .push(format!("{} -> {}", else_node_name, node_name));
        node_name.to_string()
    }
}

pub fn show_dot<T: ConvertibleToDotOutputT>(expr: &Expression, output_to: T) {
    let mut mapper = Graphvizifier { vng: make_unique_name_gen([]),
                                     node_descrs: vec![],
                                     edge_descrs: vec![],
                                     cache: HashMap::new() };
    mapper.visit(&Rc::new(expr.clone()));

    let nodes_str = mapper.node_descrs
                          .iter()
                          .fold("\n".to_string(), |acc, x| format!("{}\n{}", acc, x));

    let edges_str = mapper.edge_descrs
                          .iter()
                          .fold("\n".to_string(), |acc, x| format!("{}\n{}", acc, x));

    let dot_code = format!("digraph {{\n {}\n\n {}\n}}", nodes_str, edges_str);
    // println!("{}", dot_code);
    show_dot_code(dot_code, output_to);
}
