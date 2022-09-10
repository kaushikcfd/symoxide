use symoxide::{get_dependencies, parse};

fn main() {
    let expr = parse("x + y + (x+z)");
    let deps = get_dependencies(&expr);
    println!("Dependencies: {:?}", deps);
}
