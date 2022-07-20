use expression_trees::primitives::Variable;
use expression_trees::{variables, add};
use expression_trees::mappers::walk::{WalkMapper, WalkMappable};


struct MyWalkMapper;


impl WalkMapper for MyWalkMapper {
    fn map_variable(&self, expr: &Variable) {
        println!("Visiting '{}'.", expr.name);
    }
}


fn main() {
    let (x, y, z) = variables!("x y z");
    let expr = add(&add(&x, &y), &z);

    let var_visitor = MyWalkMapper {};
    expr.accept(&var_visitor);
}
