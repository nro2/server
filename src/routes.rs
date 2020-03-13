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
pub fn get_committees(
    conn: DbConn
) -> Result<Json<Vec<Committee>>, String> {
        use crate::schema::committee::dsl::*;

    committee.load(&conn.0).map_err(|err| -> String {
        println!("Error getting committees: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}

#[get("/committees/<id>")] 
pub fn get_committee_by_id(
    conn: DbConn, 
    id: i32
) -> Result<Json<Vec<Committee>>, String> {
        use crate::schema::committee::dsl::*;
    committee
        .filter(committee_id.eq(id))
        .load(&conn.0).map_err(|err| -> String {
        println!("Error getting committee: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}

#[post("/committees", data = "<ins_comm>")]
pub fn post_committee(
    conn: DbConn,
    ins_comm: Json<InsertCommittee>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::committee::table)
        .values(&ins_comm.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[delete("/committees/<id>")]
pub fn delete_committee(
    conn: DbConn,
    id: i32,
) -> Result<String, String> {
    use crate::schema::committee::dsl::*;
    let deleted_rows = diesel::delete(schema::committee::table)
        .filter(committee_id.eq(id))
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error deleting committee: {:?}", err);
            "Error committee row from database".into()
        })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}

#[get("/faculty")]
pub fn get_faculty(
    conn: DbConn
) -> Result<Json<Vec<Faculty>>, String> {
        use crate::schema::faculty::dsl::*;

    faculty.load(&conn.0).map_err(|err| -> String {
        println!("Error getting faculty: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}


#[get("/faculty/<qemail>")]
pub fn get_faculty_by_email(
    conn: DbConn, 
    qemail: String
) -> Result<Json<Vec<Faculty>>, String> {
        use crate::schema::faculty::dsl::*;

    faculty
        .filter(email.eq(qemail))
        .load(&conn.0).map_err(|err| -> String {
        println!("Error getting faculty: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}

#[post("/faculty", data = "<ins_fac>")]
pub fn post_faculty(
    conn: DbConn,
    ins_fac: Json<InsertFaculty>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::faculty::table)
        .values(&ins_fac.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[delete("/faculty/<qemail>")]
pub fn delete_faculty(
    conn: DbConn,
    qemail: String,
) -> Result<String, String> {
    use crate::schema::faculty::dsl::*;
    let deleted_rows = diesel::delete(schema::faculty::table)
        .filter(email.eq(qemail))
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error deleting faculty: {:?}", err);
            "Error deleting faculty from database".into()
        })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}

#[get("/department")]
pub fn get_department(conn: DbConn) -> Result<Json<Vec<Department>>, String> {
        use crate::schema::department::dsl::*;

    department.load(&conn.0).map_err(|err| -> String {
        println!("Error getting department: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}

#[get("/department/<id>")] 
pub fn get_department_by_id(
    conn: DbConn, 
    id: i32
) -> Result<Json<Vec<Department>>, String> {
        use crate::schema::department::dsl::*;
    let _id = 25;
    department
        .filter(department_id.eq(id))
        .load(&conn.0).map_err(|err| -> String {
        println!("Error getting department: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}

#[post("/department", data = "<ins_dep>")]
pub fn post_department(
    conn: DbConn,
    ins_dep: Json<InsertDepartment>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::department::table)
        .values(&ins_dep.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[delete("/department/<id>")]
pub fn delete_department(
    conn: DbConn,
    id: i32,
) -> Result<String, String> {
    use crate::schema::department::dsl::*;
    let deleted_rows = diesel::delete(schema::department::table)
        .filter(department_id.eq(id))
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error deleting department: {:?}", err);
            "Error deleting department from database".into()
        })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}