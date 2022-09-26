use hashbrown::HashMap;
use std::rc::Rc;
use symoxide::mappers::identity::IdentityMapper;
use symoxide::mappers::CachedMapper;
use symoxide::{parse, CachedMapper, Expression, ExpressionRawPointer};

#[derive(CachedMapper)]
struct Renamer {
    cache: HashMap<ExpressionRawPointer, Rc<Expression>>,
}

impl IdentityMapper for Renamer {
    fn map_variable(&mut self, name: String) -> Rc<Expression> {
        let new_name = match &name[..] {
            "x" => "foo",
            "y" => "bar",
            _ => panic!("Unknown variable {}", name),
        };
        Rc::new(Expression::Variable(new_name.to_string()))
    }
}

fn main() {
    let mut renamer = Renamer { cache: HashMap::new() };

    let xpy = parse("x+y");
    let xpy_renamed = renamer.visit(xpy.clone());
    println!("Old expr = {}", xpy);
    println!("New expr = {}", xpy_renamed);
}
