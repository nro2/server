#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate chrono;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

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
            routes::get_committee_by_id,
            routes::post_committee,
            routes::delete_committee,
            routes::get_faculty,
            routes::get_faculty_by_email,
            routes::post_faculty,
            routes::delete_faculty,
            routes::get_department,
            routes::get_department_by_id,
            routes::post_department,
            routes::delete_department])
        .attach(DbConn::fairing())
}

fn main() {
    rocket().launch();
}