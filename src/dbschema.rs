// @generated automatically by Diesel CLI.

diesel::table! {
    assigments (assigment_id) {
        assigment_id -> Nullable<Text>,
        subject_id -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    microsoft_logins (microsoft_login_id) {
        microsoft_login_id -> Nullable<Text>,
        microsoft_id -> Nullable<Text>,
        user_id -> Nullable<Text>,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Nullable<Text>,
        name -> Nullable<Text>,
        permissions -> Nullable<Integer>,
    }
}

diesel::table! {
    session_refresh_keys (refresh_key_id) {
        refresh_key_id -> Nullable<Text>,
        user_id -> Text,
        expiration_time -> Nullable<Timestamp>,
        refresh_count -> Nullable<Integer>,
        refresh_limit -> Nullable<Integer>,
    }
}

diesel::table! {
    solution (solution_id) {
        solution_id -> Nullable<Text>,
        grade -> Nullable<Double>,
        submission_date -> Nullable<Timestamp>,
        solution_data -> Nullable<Binary>,
        reviewed_by -> Nullable<Text>,
        review_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    subjects (subject_id) {
        subject_id -> Nullable<Text>,
        subject_name -> Nullable<Text>,
        editor_role_id -> Text,
    }
}

diesel::table! {
    user_solution_assignments (user_id, solution_id, assigment_id) {
        user_id -> Nullable<Text>,
        solution_id -> Nullable<Text>,
        assigment_id -> Nullable<Text>,
    }
}

diesel::table! {
    user_subjects (user_id, subject_id) {
        user_id -> Nullable<Text>,
        subject_id -> Nullable<Text>,
        role_id -> Nullable<Text>,
        grade -> Nullable<Double>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Nullable<Text>,
        email -> Text,
        name -> Text,
        surname -> Text,
        student_id -> Nullable<Text>,
        user_disabled -> Nullable<Bool>,
        last_login_time -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    assigments,
    microsoft_logins,
    roles,
    session_refresh_keys,
    solution,
    subjects,
    user_solution_assignments,
    user_subjects,
    users,
);
