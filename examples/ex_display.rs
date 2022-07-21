use expression_trees::variables;
use expression_trees::operations as ops;

fn main() {
    let (x, y, z) = variables!("x y z");

    println!("{}", ops::add(&x, &y));
    println!("{}", ops::add(&x, &z));
}
