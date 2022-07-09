use expression_trees::primitives as prim;
use expression_trees::operations as ops;

fn main() {
    let x = prim::var("x");
    let y = prim::var("y");
    let z = prim::var("z");

    println!("{}", ops::add(&x, &y));
    println!("{}", ops::add(&x, &z));
}
