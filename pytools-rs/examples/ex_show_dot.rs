use pytools_rs::show_dot;

fn main() {
    let dot_code = r#"
    digraph {
        a -> c;
        c -> d;
        b -> c;
    }
    "#;
    show_dot(dot_code.to_string(), "xwindow");
}
