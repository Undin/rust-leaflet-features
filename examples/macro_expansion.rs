#[macro_use]
extern crate diesel;

macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
            println!("You called {:?}", stringify!($func_name));
        }
    )
}

create_function!(foo);

table! {
    followings (user_id, post_id) {
        user_id -> Integer,
        post_id -> Integer,
        favorited -> Bool,
    }
}

fn main() {
    foo();
}