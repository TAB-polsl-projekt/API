// @generated automatically by Diesel CLI.

diesel::table! {
    assignments (assignment_id) {
        assignment_id -> Text,
        subject_id -> Text,
        title -> Text,
        description -> Text,
        accepted_mime_types -> Text,
    }
}

diesel::table! {
    logins (login_id) {
        login_id -> Text,
        user_id -> Text,
        email -> Text,
        passwd_hash -> Text,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Text,
        name -> Text,
    }
}

diesel::table! {
    session_ids (refresh_key_id) {
        refresh_key_id -> Text,
        user_id -> Text,
        expiration_time -> Timestamp,
    }
}

diesel::table! {
    solutions (solution_id) {
        solution_id -> Text,
        grade -> Nullable<Double>,
        submission_date -> Timestamp,
        solution_data -> Binary,
        reviewed_by -> Nullable<Text>,
        review_comment -> Nullable<Text>,
        student_comment -> Nullable<Text>,
        exercise_date -> Nullable<Timestamp>,
        review_date -> Nullable<Timestamp>,
        mime_type -> Text,
        assignment_id -> Text,
    }
}

diesel::table! {
    subject_role (subject_id, role_id) {
        subject_id -> Text,
        role_id -> Text,
    }
}

diesel::table! {
    subjects (subject_id) {
        subject_id -> Text,
        subject_name -> Text,
        editor_role_id -> Nullable<Text>,
    }
}

diesel::table! {
    user_role (role_id, user_id) {
        role_id -> Text,
        user_id -> Text,
    }
}

diesel::table! {
    user_solution (user_id, solution_id) {
        user_id -> Text,
        solution_id -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Text,
        name -> Text,
        surname -> Text,
        student_id -> Nullable<Text>,
        is_admin -> Bool,
    }
}

diesel::joinable!(assignments -> subjects (subject_id));
diesel::joinable!(logins -> users (user_id));
diesel::joinable!(session_ids -> users (user_id));
diesel::joinable!(solutions -> assignments (assignment_id));
diesel::joinable!(solutions -> users (reviewed_by));
diesel::joinable!(subject_role -> roles (role_id));
diesel::joinable!(subject_role -> subjects (subject_id));
diesel::joinable!(subjects -> roles (editor_role_id));
diesel::joinable!(user_role -> roles (role_id));
diesel::joinable!(user_role -> users (user_id));
diesel::joinable!(user_solution -> solutions (solution_id));
diesel::joinable!(user_solution -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    assignments,
    logins,
    roles,
    session_ids,
    solutions,
    subject_role,
    subjects,
    user_role,
    user_solution,
    users,
);
