use expression_trees::{variables, add};

fn main() {
    let (x, y) = variables!("x y");
    println!("{}", add(&x, &y));
}
