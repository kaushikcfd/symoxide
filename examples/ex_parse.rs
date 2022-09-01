use env_logger;
use symoxide::parse::parse_expr;

fn main() {
    env_logger::init();
    let code = "foo(x[0.2343242343e4, i, 1], -2*x)";
    let expr = parse_expr(&code[..]);
    println!("Parsed as {}", expr);
}
