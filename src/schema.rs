table! {
    committee (committee_id) {
        committee_id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    committee_assignment (email, committee_id) {
        email -> Varchar,
        committee_id -> Int4,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
    }
}

table! {
    department (department_id) {
        department_id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    department_association (email, department_id) {
        email -> Varchar,
        department_id -> Int4,
    }
}

table! {
    faculty (email) {
        faculty_id -> Int4,
        full_name -> Varchar,
        email -> Varchar,
        job_title -> Nullable<Varchar>,
        senate_division -> Varchar,
    }
}

table! {
    survey_choice (choice_id, email, survey_date, committee_id) {
        choice_id -> Int4,
        survey_date -> Date,
        email -> Varchar,
        committee_id -> Int4,
    }
}

table! {
    survey_data (survey_date, email) {
        survey_date -> Date,
        email -> Varchar,
        is_interested -> Bool,
        expertise -> Nullable<Varchar>,
    }
}

joinable!(committee_assignment -> committee (committee_id));
joinable!(committee_assignment -> faculty (email));
joinable!(department_association -> department (department_id));
joinable!(department_association -> faculty (email));
joinable!(survey_choice -> committee (committee_id));
joinable!(survey_choice -> faculty (email));
joinable!(survey_data -> faculty (email));

allow_tables_to_appear_in_same_query!(
    committee,
    committee_assignment,
    department,
    department_association,
    faculty,
    survey_choice,
    survey_data,
);
