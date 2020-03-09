use diesel::sql_types::Date;
use diesel::sql_types::Bool;

#[derive(Queryable)]
pub struct Committee {
    pub committee_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Queryable)]
pub struct CommitteeAssignment {
    pub email: String,
    pub committee_id: i32,
    pub start_date: Date,
    pub end_date: Date,
}

#[derive(Queryable)]
pub struct Department {
    pub department_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Queryable)]
pub struct DepartmentAssociations {
    pub email: String,
    pub department_id: i32,
}

#[derive(Queryable)]
pub struct Faculty {
    pub faculty_id: i32,
    pub full_name: String,
    pub email: String,
    pub job_title: String,
    pub senate_division: String,
}

#[derive(Queryable)]
pub struct SurveyChoice {
    pub choice_id: i32,
    pub survey_date: Date,
    pub email: String,
    pub committee_id: i32,
}

#[derive(Queryable)]
pub struct SurveyData {
    pub survey_date: Date,
    pub email: String,
    pub is_interested: Bool,
    pub expertise: String,
}
