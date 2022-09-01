use symoxide::{add, variables};

fn main() {
    let (x, y) = variables!("x y");
    println!("{}", add(&x, &y));
}
