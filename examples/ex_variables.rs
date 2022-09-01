use symoxide::{variables, add};

fn main() {
    let (x, y) = variables!("x y");
    println!("{}", add(&x, &y));
}
