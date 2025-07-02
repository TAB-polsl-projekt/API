use rocket::Route;
use rocket_okapi::{okapi::{merge::marge_spec_list, openapi3::OpenApi}, settings::OpenApiSettings};

mod account_get;
mod account_put;
mod assignment_get;
mod assignment_put;
mod assignment_solution_get;
mod assignment_solution_post;
mod assignments_post;
mod auth_delete;
mod auth_post;
mod subject_get;
mod subject_put;
mod subjects_get;
mod subjects_post;
mod users_get;
mod assignments_get;
mod get_solutions;
mod subjects_delete;
mod assignment_delete;
mod subject_role_post;
mod subject_role_delete;
mod subject_student_assignments_get;
mod subject_solutions_get;
mod enrolled;
mod idk;
mod user_roles_get;
mod user_role_post;
mod user_role_delete;
mod subject_roles_get;
mod account_post;
mod roles;
mod users;
mod subject_teachers_get;
mod assignment_attendance_get;
mod assignment_attendance_post;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    // Start with an empty vector for routes and an initial empty OpenAPI object.
    let mut all_routes_and_docs = Vec::new();
    let prefix = "".to_string();

    all_routes_and_docs.push(assignment_solution_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(account_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(account_put::get_routes_and_docs(settings));
    all_routes_and_docs.push(subject_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(subject_put::get_routes_and_docs(settings));
    all_routes_and_docs.push(subjects_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(assignment_get::get_routes_and_docs(settings));
    // all_routes_and_docs.push(assignment_solution_post::get_routes_and_docs(settings));
    all_routes_and_docs.push(assignments_post::get_routes_and_docs(settings));
    all_routes_and_docs.push(assignment_put::get_routes_and_docs(settings));
    all_routes_and_docs.push(auth_delete::get_routes_and_docs(settings));
    all_routes_and_docs.push(auth_post::get_routes_and_docs(settings));
    all_routes_and_docs.push(users_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(assignments_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(get_solutions::get_routes_and_docs(settings));
    all_routes_and_docs.push(subjects_post::get_routes_and_docs(settings));
    all_routes_and_docs.push(subjects_delete::get_routes_and_docs(settings));
    all_routes_and_docs.push(assignment_delete::get_routes_and_docs(settings));
    all_routes_and_docs.push(subject_role_post::get_routes_and_docs(settings));
    all_routes_and_docs.push(subject_role_delete::get_routes_and_docs(settings));
    all_routes_and_docs.push(subject_solutions_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(subject_student_assignments_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(enrolled::get_routes_and_docs(settings));
    all_routes_and_docs.push(idk::get_routes_and_docs(settings));
    all_routes_and_docs.push(user_roles_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(user_role_post::get_routes_and_docs(settings));
    all_routes_and_docs.push(user_role_delete::get_routes_and_docs(settings));
    all_routes_and_docs.push(subject_roles_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(account_post::get_routes_and_docs(settings));
    all_routes_and_docs.push(roles::get_routes_and_docs(settings));
    all_routes_and_docs.push(users::get_routes_and_docs(settings));
    all_routes_and_docs.push(subject_teachers_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(assignment_attendance_get::get_routes_and_docs(settings));
    all_routes_and_docs.push(assignment_attendance_post::get_routes_and_docs(settings));

    let (mut all_routes, all_spec) = all_routes_and_docs.into_iter()
        .fold((Vec::new(), Vec::new()), |(mut routes, mut specs), it| {
            routes.extend(it.0); specs.push(it.1); (routes, specs)
        });

    all_routes.append(&mut assignment_solution_post::get_routes());

    let spec_list: Vec<(&String, OpenApi)> = all_spec.into_iter().map(|x| (&prefix, x)).collect();
    let merged_spec = marge_spec_list(&spec_list).unwrap();

    (all_routes, merged_spec)
}
