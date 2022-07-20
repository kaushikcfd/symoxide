use expression_trees::{variables, add};
use expression_trees::mappers::combine::CombineMappable;
use expression_trees::mapper_impls::dependency::DependenciesGetter;


fn main() {
    let (x, y, z) = variables!("x y z");
    let expr = add(&add(&x, &y), &add(&x, &z));
    let dep_mapper = DependenciesGetter {};

    let deps = expr.accept(&dep_mapper);
    println!("Dependencies: {:?}", deps);
}
