#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate dotenv;
extern crate chrono;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket::http::RawStr;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::*;
use self::diesel::prelude::*;

//pub mod cors;
pub mod schema;
pub mod models;
pub mod routes;

#[database("coc")]
pub struct DbConn(diesel::PgConnection);

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            routes::index,
            routes::get_committees,
            routes::get_faculty])
        .attach(DbConn::fairing())
}

fn main() {
    rocket().launch();
}