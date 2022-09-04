use symoxide::mappers::walk::UncachedWalkMapper;
use symoxide::{add, variables};

struct MyWalkMapper;

impl UncachedWalkMapper for MyWalkMapper {
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
