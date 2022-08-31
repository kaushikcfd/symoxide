use expression_trees::parse::parse_expr;
use env_logger;


fn main() {
    env_logger::init();
    // let expr = parse_expr(" ( (x))  ");
    // println!("Parsed as {}", expr);
    // Using this to measure the time in micro seconds
    let n = 1_000_000;

    for i in 0..n {
        let expr = parse_expr("x + z*y");
        if i == (n-1) {
            println!("Parsed as {}", expr);
        }
    }
}
