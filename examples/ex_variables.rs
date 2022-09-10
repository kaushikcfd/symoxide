use symoxide::operations as ops;
use symoxide::variables;

fn main() {
    let (x, y) = variables!("x y");
    println!("{}", ops::add(&x, &y));
}
