use pytools_rs::make_unique_name_gen;

fn main() {
    let mut vng = make_unique_name_gen([]);
    println!("vng.get(foo) => {}", vng.get("foo"));
    println!("vng.get(foo) => {}", vng.get("foo"));
    println!("vng.get(bar_2) => {}", vng.get("bar_2"));
    println!("vng.get(bar_2) => {}", vng.get("bar_2"));
    println!("vng.get(bar_2) => {}", vng.get("bar_2"));
}
