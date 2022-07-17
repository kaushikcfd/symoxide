use expression_trees::mappers::identity::{IdentityMapper, IdentityMappable};
use expression_trees::{variables, add};

struct Copier;
impl IdentityMapper for Copier {}



fn main() {

    let copier = Copier {};
    let (x, y) = variables!("x y");

    let xpy = add(&x, &y);
    let xpy_copy = xpy.accept(&copier);
    // TODO: Once Expression to be forced of type Display uncomment the line below.
    //println!("{}", xpy_copy);
}
