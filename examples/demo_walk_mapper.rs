use expression_trees::{variables, add};
use expression_trees::mappers::walk::{WalkMapper};


struct MyWalkMapper;


impl WalkMapper for MyWalkMapper {
    fn map_variable(&self, name: String) {
        println!("Visiting '{}'.", name);
    }
}


fn main() {
    let (x, y, z) = variables!("x y z");
    let expr = add(&add(&x, &y), &z);

    let var_visitor = MyWalkMapper {};
    var_visitor.visit(&expr);
}
