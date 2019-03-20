#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};

#[get("/")]
fn hello() -> String {
    "hello".to_string()
}
#[get("/<name>/<age>")]
fn name_age(name: String, age: usize) -> String {
    format!("hello {}. age: {}", name, age)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
