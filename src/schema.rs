// @generated automatically by Diesel CLI.

diesel::table! {
    Assigments (assigment_id) {
        assigment_id -> Nullable<Text>,
        subject_id -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    Microsoft_Logins (microsoft_login_id) {
        microsoft_login_id -> Nullable<Text>,
        microsoft_id -> Nullable<Text>,
        user_id -> Nullable<Text>,
    }
}

diesel::table! {
    Roles (role_id) {
        role_id -> Nullable<Text>,
        name -> Nullable<Text>,
        permissions -> Nullable<Integer>,
    }
}

diesel::table! {
    Session_Refresh_Keys (refresh_key_id) {
        refresh_key_id -> Nullable<Text>,
        user_id -> Text,
        expiration_time -> Nullable<Timestamp>,
        refresh_count -> Nullable<Integer>,
        refresh_limit -> Nullable<Integer>,
    }
}

diesel::table! {
    Solution (solution_id) {
        solution_id -> Nullable<Text>,
        grade -> Nullable<Double>,
        submission_date -> Nullable<Timestamp>,
        solution_data -> Nullable<Binary>,
        reviewed_by -> Nullable<Text>,
        review_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    Subjects (subject_id) {
        subject_id -> Nullable<Text>,
        subject_name -> Nullable<Text>,
        editor_role_id -> Text,
    }
}

diesel::table! {
    User_Solution_Assignments (user_id, solution_id, assigment_id) {
        user_id -> Nullable<Text>,
        solution_id -> Nullable<Text>,
        assigment_id -> Nullable<Text>,
    }
}

diesel::table! {
    User_Subjects (user_id, subject_id) {
        user_id -> Nullable<Text>,
        subject_id -> Nullable<Text>,
        role_id -> Nullable<Text>,
        grade -> Nullable<Double>,
    }
}

diesel::table! {
    Users (user_id) {
        user_id -> Nullable<Text>,
        email -> Text,
        name -> Text,
        surname -> Text,
        student_id -> Nullable<Text>,
        user_disabled -> Nullable<Bool>,
        last_login_time -> Nullable<Timestamp>,
    }
}

diesel::joinable!(Assigments -> Subjects (subject_id));
diesel::joinable!(Microsoft_Logins -> Users (user_id));
diesel::joinable!(Session_Refresh_Keys -> Users (user_id));
diesel::joinable!(Solution -> Users (reviewed_by));
diesel::joinable!(Subjects -> Roles (editor_role_id));
diesel::joinable!(User_Solution_Assignments -> Assigments (assigment_id));
diesel::joinable!(User_Solution_Assignments -> Solution (solution_id));
diesel::joinable!(User_Solution_Assignments -> Users (user_id));
diesel::joinable!(User_Subjects -> Roles (role_id));
diesel::joinable!(User_Subjects -> Subjects (subject_id));
diesel::joinable!(User_Subjects -> Users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    Assigments,
    Microsoft_Logins,
    Roles,
    Session_Refresh_Keys,
    Solution,
    Subjects,
    User_Solution_Assignments,
    User_Subjects,
    Users,
);
