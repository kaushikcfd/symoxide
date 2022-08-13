use expression_trees::parse::parse_expr;


fn main() {
    let expr = parse_expr("(((x))");
    println!("Parsed as {}", expr);
}
