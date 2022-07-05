use expression_trees::primitives as prim;

fn main() {
    let x = prim::var("x");
    let y = prim::var("y");
    let z = prim::var("z");

    println!("{}", prim::add(&x, &y));
    println!("{}", prim::add(&x, &z));
    // println!("{}", x+2);
    // println!("{}", x*2);
}
