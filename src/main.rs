#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "made it to the second endpoint!"
}

#[get("/world/<name>")]
fn world_hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/world/<name>/<age>/<cool>")]
fn world_hello_age(name: String, age:u8, cool: bool) -> String {
    if cool {
        format!("Your name is {}, you are {} years old and you are cool", name, age)
    }
    else {
        format!("Your age is {}, but you aren't very cool {}", age, name)
    }
}
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/hello", routes![world, world_hello, world_hello_age])
}

fn main() {
    rocket().launch();
}