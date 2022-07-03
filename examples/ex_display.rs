use expression_trees::primitives as prim;

fn main() {
    let x = prim::var("x");
    println!("{}", x+2);
}
