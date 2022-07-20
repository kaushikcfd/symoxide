use expression_trees::mappers::fold::{FoldMapper, Foldable};
use expression_trees::primitives::{Variable, Expression, Sum};
use expression_trees::{variables, add};
use std::rc::Rc;

struct Renamer;

impl FoldMapper for Renamer {
    type Output = Rc<dyn Expression>;

    fn map_variable(&self, expr: &Variable) -> Rc<dyn Expression> {
        let new_name = match &expr.name[..] {
            "x" => "foo",
            "y" => "bar",
            _  => panic!("Unknown variable {}", expr),
        };
        Rc::new(Variable  {name: new_name.to_string()})
    }

    fn map_sum<T1: Foldable, T2: Foldable>(&self, expr: &Sum<T1, T2>) -> Rc<dyn Expression>{
        let rec_l = expr.l.accept(self);
        let rec_r = expr.r.accept(self);
        return Rc::new(Sum {l: rec_l, r: rec_r});
    }
}


fn main() {

    let renamer = Renamer {};
    let (x, y) = variables!("x y");

    let xpy = add(&x, &y);
    let xpy_renamed = xpy.accept(&renamer);
    println!("Old expr = {}", xpy);
    println!("New expr = {}", xpy_renamed);
}
