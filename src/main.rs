#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search")]
fn search() -> &'static str {
    "Hello, search!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}
