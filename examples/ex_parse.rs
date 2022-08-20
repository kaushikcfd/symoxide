use expression_trees::parse::parse_expr;
use env_logger;


fn main() {
    env_logger::init();
    // let expr = parse_expr(" ( (x))  ");
    // println!("Parsed as {}", expr);
    let expr = parse_expr("x");
    println!("Parsed as {}", expr);
}
