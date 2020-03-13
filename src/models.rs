// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use chrono::NaiveDate;
use crate::schema::*;


#[derive(Queryable, Debug, Identifiable,Serialize, Deserialize)]
#[primary_key(committee_id)]
#[table_name="committee"]
pub struct Committee {
    pub committee_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Debug, Identifiable,Serialize, Deserialize)]
#[primary_key(email, committee_id)]
#[table_name="committee_assignment"]
pub struct CommitteeAssignment {
    pub email: String,
    pub committee_id: i32,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Queryable, Debug, Identifiable,Serialize, Deserialize)]
#[primary_key(department_id)]
#[table_name="department"]
pub struct Department {
    pub department_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Debug, Identifiable,Serialize, Deserialize)]
#[primary_key(email, department_id)]
#[table_name="department_association"]
pub struct DepartmentAssociation {
    pub email: String,
    pub department_id: i32,
}

#[derive(Queryable, Debug, Identifiable,Serialize, Deserialize)]
#[primary_key(email)]
#[table_name="faculty"]
pub struct Faculty {
    pub faculty_id: i32,
    pub full_name: String,
    pub email: String,
    pub job_title: Option<String>,
    pub senate_division: String,
}

#[derive(Queryable, Debug, Identifiable,Serialize, Deserialize)]
#[primary_key(choice_id, email, survey_date, committee_id)]
#[table_name="survey_choice"]
pub struct SurveyChoice {
    pub choice_id: i32,
    pub survey_date: NaiveDate,
    pub email: String,
    pub committee_id: i32,
}

#[derive(Queryable, Debug, Identifiable,Serialize, Deserialize)]
#[primary_key(survey_date, email)]
#[table_name="survey_data"]
pub struct SurveyData {
    pub survey_date: NaiveDate,
    pub email: String,
    pub is_interested: bool,
    pub expertise: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="committee"]
pub struct InsertCommittee {
    pub committee_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name="faculty"]
pub struct InsertFaculty {
    pub faculty_id: i32,
    pub full_name: String,
    pub email: String,
    pub job_title: Option<String>,
    pub senate_division: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="department"]
pub struct InsertDepartment {
    pub department_id: i32,
    pub name: String,
    pub description: Option<String>,
}