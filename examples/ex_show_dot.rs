use symoxide as sym;

fn main() {
    let expr = sym::parse("foo[bar, baz, sin(x)]");
    sym::show_dot(&expr, "xwindow");
}
