use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::schema::{assignments, subjects, subject_role, user_role};
use crate::define_api_response;
use crate::admin_session::AdminSession;
use crate::dbmodels::Assignment;
use diesel::BoolExpressionMethods;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: student_assignments]
}

define_api_response!(pub enum Response {
    Ok => (200, "Returns student assignments", Vec<(String, Assignment)>, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "Internal Server Error", (), (diesel::result::Error)),
});

#[openapi(tag = "Subjects", operation_id = "student_assignments")]
#[get("/subjects/<subject_id>/student-assignments")]
pub async fn student_assignments(
    subject_id: String,
    conn: crate::db::DbConn,
    _session: AdminSession
) -> Result<Response, Error> {
    let data = conn.run(move |c| {
        // Join subjects -> assignments for this subject
        // Then find all users with 'student' role for the subject
        subjects::table
            .filter(subjects::subject_id.eq(&subject_id))
            .inner_join(assignments::table.on(assignments::subject_id.eq(subjects::subject_id)))
            .inner_join(
                subject_role::table.on(
                    subject_role::subject_id.eq(subjects::subject_id)
                    .and(subject_role::role_id.eq("student"))
                )
            )
            .inner_join(
                user_role::table.on(
                    user_role::role_id.eq(subject_role::role_id)
                )
            )
            .select((user_role::user_id, assignments::all_columns))
            .get_results::<(String, Assignment)>(c)
    }).await?;

    Ok(Response::Ok(data))
}
