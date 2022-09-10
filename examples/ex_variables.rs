use symoxide::variables;
use symoxide::operations as ops;

fn main() {
    let (x, y) = variables!("x y");
    println!("{}", ops::add(&x, &y));
}
