use std::rc::Rc;
use symoxide::scalar;

fn main() {
    let a: Rc<symoxide::Expression> = scalar!(5.42e2);
    println!("{:?}", a);

    let b = scalar!(5);
    println!("{:?}", b);

    let c = scalar!(-5);
    println!("{:?}", c);
}
