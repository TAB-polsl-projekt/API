use chrono::Utc;
use diesel::dsl::exists;
use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::post;
use rocket::serde::uuid::Uuid;
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use crate::define_api_response;
use crate::session::Session;

use crate::dbmodels::{Assignment, Role, Solution, Subject, SubjectRole, User, UserRole};
use crate::schema::{assignments, roles, solutions, subject_role, subjects, user_role, users};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "", (), ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "User does not have access to the assignment", (), ()),
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

#[openapi(tag = "Assignments", operation_id = "postAssignmetSolution")]
#[post("/assignments/<assignment_id>/solution", data = "<sln>")]
pub async fn endpoint(assignment_id: String, sln: Json<Solution>, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let mut sln = sln.0;
    let user_id = session.user_id;

    let result = conn.run(move |c| {

        let user: User = users::table.find(user_id).first(c)?;

        let roles: Vec<Role> = UserRole::belonging_to(&user)
            .inner_join(roles::table.on(roles::role_id.eq(user_role::role_id)))
            .select(roles::all_columns)
            .load(c)?;

        let subject: Subject = SubjectRole::belonging_to(&roles)
            .inner_join(subjects::table.on(subjects::subject_id.eq(subject_role::subject_id)))
            .select(subjects::all_columns)
            .first(c)?;

        let assignment_query = Assignment::belonging_to(&subject)
            .filter(assignments::assignment_id.eq(assignment_id));

        let user_has_access_to_assignments: bool = diesel::select(exists(assignment_query))
            .get_result(c)?;

        if !user_has_access_to_assignments {
            return Err(Error::Unauthorized(()));
        }

        sln.solution_id = Uuid::new_v4().to_string();
        sln.submission_date = Utc::now().naive_utc();
        sln.assignment_id = assignment_id;

        let _result = diesel::insert_into(solutions::table)
            .values(sln)
            .execute(c)?;

        Ok(())
    })
    .await?;

    Ok(Response::Ok(result))
}