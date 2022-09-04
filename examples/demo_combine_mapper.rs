use symoxide::{add, get_dependencies, variables};

fn main() {
    let (x, y, z) = variables!("x y z");
    let expr = add(&add(&x, &y), &add(&x, &z));
    let deps = get_dependencies(&expr);
    println!("Dependencies: {:?}", deps);
}
