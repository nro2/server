#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "made it to the second endpoint!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/hello", routes![world])
}

fn main() {
    rocket().launch();
}