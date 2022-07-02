use expression_trees::primitives as prim;


fn main() {
    let x = prim::var("x");
    let y = prim::var("y");
    println!("{}", 2*x + 2*y);
}
