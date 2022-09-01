use symoxide as sym;
use symoxide::parse::parse_expr as parse;

#[test]
fn test_parser() {
    let expr = parse(concat!("(2*a[1]*b[1]+2*a[0]*b[0])*(hankel_1(-1,sqrt(a[1]**2+a[0]**2)*k) ",
                      "-hankel_1(1,sqrt(a[1]**2+a[0]**2)*k))*k /(4*sqrt(a[1]**2+a[0]**2)) ",
                      "+hankel_1(0,sqrt(a[1]**2+a[0]**2)*k)"));
    assert_eq!(parse("d4knl0"), sym::var("d4knl0"));
}
