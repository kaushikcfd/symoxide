use symoxide::parse;

fn main() {
    let expr = parse("a+b*c");
    println!("{}", expr);
    let expr = parse("(a+b)*c");
    println!("{}", expr);
    let expr = parse("a+b+c");
    println!("{}", expr);
    let expr = parse("a+(b+c)");
    println!("{}", expr);
    let expr = parse("(a+b)+3.14");
    println!("{}", expr);
}
