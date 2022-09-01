use symoxide::{variables, add};
use symoxide::mappers::combine::CombineMapper;
use symoxide::mapper_impls::dependency::DependenciesGetter;


fn main() {
    let (x, y, z) = variables!("x y z");
    let expr = add(&add(&x, &y), &add(&x, &z));
    let dep_mapper = DependenciesGetter {};

    let deps = dep_mapper.visit(&expr);
    println!("Dependencies: {:?}", deps);
}
