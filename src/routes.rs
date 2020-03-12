use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::*;
use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> &'static str {
    "Application running"
}

#[get("/committees")]
pub fn get_committees(conn: DbConn) -> Result<Json<Vec<Committee>>, String> {
        use crate::schema::committee::dsl::*;

    committee.load(&conn.0).map_err(|err| -> String {
        println!("Error getting committees: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}

#[get("/committees/<id>")] 
pub fn get_committee_by_id(conn: DbConn, id: i32) -> Result<Json<Vec<Committee>>, String> {
        use crate::schema::committee::dsl::*;
    let _id = 25;
    committee
        .filter(committee_id.eq(id))
        .load(&conn.0).map_err(|err| -> String {
        println!("Error getting committee: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}

#[get("/faculty")]
pub fn get_faculty(conn: DbConn) -> Result<Json<Vec<Faculty>>, String> {
        use crate::schema::faculty::dsl::*;

    faculty.load(&conn.0).map_err(|err| -> String {
        println!("Error getting faculty: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}

#[get("/faculty/<email>")]
pub fn get_faculty_by_email(conn: DbConn, email: String) -> Result<Json<Vec<Faculty>>, String> {
        use crate::schema::faculty::dsl::*;

    faculty
        .filter(email.eq(email))
        .load(&conn.0).map_err(|err| -> String {
        println!("Error getting faculty: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}
