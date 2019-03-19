#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};

#[get("/")]
fn hello() -> String {
    "hello".to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
