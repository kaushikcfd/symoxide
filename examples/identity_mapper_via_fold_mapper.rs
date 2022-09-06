use std::rc::Rc;
use symoxide::mappers::fold::UncachedFoldMapper as FoldMapper;
use symoxide::{add, variables};
use symoxide::{BinaryOpType, Expression, ScalarT, UnaryOpType};

struct Renamer;

impl FoldMapper for Renamer {
    type Output = Rc<Expression>;

    fn map_scalar(&self, expr: &ScalarT) -> Rc<Expression> {
        return Rc::new(Expression::Scalar(expr.clone()));
    }

    fn map_variable(&self, name: String) -> Rc<Expression> {
        let new_name = match &name[..] {
            "x" => "foo",
            "y" => "bar",
            _ => panic!("Unknown variable {}", name),
        };
        Rc::new(Expression::Variable(new_name.to_string()))
    }

    fn map_binary_op(&self, left: &Expression, op: BinaryOpType, right: &Expression)
                     -> Rc<Expression> {
        let rec_l = self.visit(left);
        let rec_r = self.visit(right);
        return Rc::new(Expression::BinaryOp(rec_l, op.clone(), rec_r));
    }

    fn map_unary_op(&self, op: UnaryOpType, x: &Expression) -> Rc<Expression> {
        return Rc::new(Expression::UnaryOp(op.clone(), self.visit(x)));
    }

    fn map_call(&self, call: &Expression, params: &Vec<Rc<Expression>>) -> Rc<Expression> {
        let rec_params = params.iter().map(|par| self.visit(par)).collect();
        return Rc::new(Expression::Call(self.visit(call), rec_params));
    }

    fn map_subscript(&self, agg: &Expression, indices: &Vec<Rc<Expression>>) -> Rc<Expression> {
        let rec_indices = indices.iter().map(|idx| self.visit(idx)).collect();
        return Rc::new(Expression::Subscript(self.visit(agg), rec_indices));
    }

    fn map_if(&self, cond: &Expression, then: &Expression, else_: &Expression)
              -> Rc<Expression> {
        let rec_cond = self.visit(cond);
        let rec_then = self.visit(then);
        let rec_else = self.visit(else_);
        return Rc::new(Expression::If(rec_cond, rec_then, rec_else));
    }
}

fn main() {
    let renamer = Renamer {};
    let (x, y) = variables!("x y");

    let xpy = add(&x, &y);
    let xpy_renamed = renamer.visit(&xpy);
    println!("Old expr = {}", xpy);
    println!("New expr = {}", xpy_renamed);
}
