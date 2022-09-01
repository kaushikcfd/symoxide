use symoxide::operations as ops;
use symoxide::variables;

fn main() {
    let (x, y, z) = variables!("x y z");

    println!("{}", ops::add(&x, &y));
    println!("{}", ops::add(&x, &z));
}
