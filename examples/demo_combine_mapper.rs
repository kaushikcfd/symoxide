use symoxide::{parse, get_dependencies};

fn main() {
    let expr = parse("x + y + (x+z)");
    let deps = get_dependencies(&expr);
    println!("Dependencies: {:?}", deps);
}
