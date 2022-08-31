use expression_trees::parse::parse_expr;
use env_logger;


fn main() {
    env_logger::init();
    // let expr = parse_expr(" ( (x))  ");
    // println!("Parsed as {}", expr);
    // Using this to measure the time in micro seconds
    let n = 10;

    for i in 0..n {
        let code = format!("x + z*(y + a{})", i);
        let expr = parse_expr(&code[..]);
        println!("Parsed as {}", expr);
    }
}
