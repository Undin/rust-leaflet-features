use std::sync::Mutex;

fn main() {
    // auto import
    let counter = Arc::new(Mutex::new(0));

    // replace `unwrap` with match
    let x: i32 = "123".parse().unwrap();
}

struct Point {
    x: i32,
    y: i32
}

// implement members
impl From<(i32, i32)> for Point {

}

// elide lifetimes
fn process<'a>(s: &'a str) {

}