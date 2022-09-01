use symoxide::mapper_impls::dependency::DependenciesGetter;
use symoxide::mappers::combine::CombineMapper;
use symoxide::{add, variables};

fn main() {
    let (x, y, z) = variables!("x y z");
    let expr = add(&add(&x, &y), &add(&x, &z));
    let dep_mapper = DependenciesGetter {};

    let deps = dep_mapper.visit(&expr);
    println!("Dependencies: {:?}", deps);
}
