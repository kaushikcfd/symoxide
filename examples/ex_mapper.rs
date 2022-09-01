use std::rc::Rc;
use symoxide::mappers::identity::IdentityMapper;
use symoxide::Expression;
use symoxide::{add, variables};

struct Renamer;
impl IdentityMapper for Renamer {
    fn map_variable(&self, name: String) -> Rc<Expression> {
        let new_name = match &name[..] {
            "x" => "foo",
            "y" => "bar",
            _ => panic!("Unknown variable {}", name),
        };
        Rc::new(Expression::Variable(new_name.to_string()))
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
