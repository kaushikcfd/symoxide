use symoxide as sym;

fn main() {
    let expr = sym::parse("foo[bar, baz, sin(x)] * cos(x)");
    sym::show_dot(&expr, "xwindow");
}
