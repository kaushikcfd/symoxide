use std::rc::Rc;
use symoxide::mappers::fold::FoldMapper;
use symoxide::{add, variables};
use symoxide::{BinaryOpType, Expression, ScalarT};

struct Renamer;

impl FoldMapper for Renamer {
    type Output = Rc<Expression>;

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

    fn map_scalar(&self, expr: &ScalarT) -> Rc<Expression> {
        return Rc::new(Expression::Scalar(expr.clone()));
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
