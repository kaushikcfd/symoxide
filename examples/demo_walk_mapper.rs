use symoxide::mappers::walk::UncachedWalkMapper;
use symoxide::parse;

struct MyWalkMapper;

impl UncachedWalkMapper for MyWalkMapper {
    fn map_variable(&self, name: String) {
        println!("Visiting '{}'.", name);
    }
}

fn main() {
    let expr = parse("x + y + z");

    let var_visitor = MyWalkMapper {};
    var_visitor.visit(&expr);
}
