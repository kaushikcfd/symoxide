use expression_trees::primitives as prim;

fn main() {
    let x = prim::var("x");
    // println!("{}", x+2);
    // println!("{}", x*2);
    println!("{}", 2/x);
}
