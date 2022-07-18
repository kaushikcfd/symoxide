use expression_trees::mappers::identity::{IdentityMapper, IdentityMappable};
use expression_trees::primitives::{Variable, Expression};
use expression_trees::{variables, add};
use std::rc::Rc;

struct Renamer;
impl IdentityMapper for Renamer {
    fn map_variable(&self, expr: &Variable) -> Rc<dyn Expression> {
        let new_name = match &expr.name[..] {
            "x" => "foo",
            "y" => "bar",
            _  => panic!("Unknown variable {}", expr),
        };
        Rc::new(Variable  {name: new_name.to_string()})
    }
}


fn main() {

    let renamer = Renamer {};
    let (x, y) = variables!("x y");

    let xpy = add(&x, &y);
    let xpy_renamed = xpy.accept(&renamer);
    println!("{}", xpy_renamed);
}
