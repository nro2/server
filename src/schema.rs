table! {
    committees (committee_id) {
        committee_id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    committee_assignments (email, committee_id) {
        email -> Varchar,
        committee_id -> Int4,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
    }
}

table! {
    departments (department_id) {
        department_id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    department_associations (email, department_id) {
        email -> Varchar,
        department_id -> Int4,
    }
}

table! {
    facultys (email) {
        faculty_id -> Int4,
        full_name -> Varchar,
        email -> Varchar,
        job_title -> Nullable<Varchar>,
        senate_division -> Varchar,
    }
}

table! {
    survey_choices (choice_id, email, survey_date, committee_id) {
        choice_id -> Int4,
        survey_date -> Date,
        email -> Varchar,
        committee_id -> Int4,
    }
}

table! {
    survey_datas (survey_date, email) {
        survey_date -> Date,
        email -> Varchar,
        is_interested -> Bool,
        expertise -> Nullable<Varchar>,
    }
}

joinable!(committee_assignments -> committees (committee_id));
joinable!(committee_assignments -> facultys (email));
joinable!(department_associations -> departments (department_id));
joinable!(department_associations -> facultys (email));
joinable!(survey_choices -> committees (committee_id));
joinable!(survey_choices -> facultys (email));
joinable!(survey_datas -> facultys (email));

allow_tables_to_appear_in_same_query!(
    committees,
    committee_assignments,
    departments,
    department_associations,
    facultys,
    survey_choices,
    survey_datas,
);
