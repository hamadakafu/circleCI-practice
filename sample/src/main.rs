#![feature(proc_macro_hygiene, decl_macro)]
mod some {
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_add_one() {
            assert_eq!(3, add_one(2));
        }
    }
    fn add_one(x: isize) -> isize {
        x + 1
    }
}
use rocket::{routes, get};
#[get("/")]
fn hello() -> String {
    "hello, be built".to_string()
}
fn main() {
    rocket::ignite().mount("/", routes![
        hello,
    ]).launch();
}
