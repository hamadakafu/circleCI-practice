#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{
    get, routes, http::RawStr,
};

#[get("/")]
fn hello() -> String {
    "hello".to_string()
}
#[get("/<name>/<age>")]
fn name_age(name: &RawStr, age: usize) -> String {
    format!("hello {}. age: {}", name, age)
}

fn main() {
    rocket::ignite().mount("/", routes![hello, name_age]).launch();
}
