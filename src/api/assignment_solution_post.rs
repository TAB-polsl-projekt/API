use chrono::Utc;
use diesel::dsl::exists;
use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::post;
use rocket::serde::uuid::Uuid;
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use crate::define_api_response;
use crate::session::Session;

use crate::dbmodels::{Solution, User, UserRole, UserSolution};
use crate::schema::{assignments, roles, solutions, subject_role, subjects, user_role, user_solution, users};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "", (), ()),
});

define_api_response!(pub enum Error {
    Forbidden => (403, "User does not belong to the subject", (), ()),
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

#[openapi(tag = "Assignments", operation_id = "postAssignmetSolution")]
#[post("/assignments/<assignment_id>/solution", data = "<sln>")]
pub async fn endpoint(assignment_id: String, sln: Json<Solution>, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let mut sln = sln.0;
    let user_id = session.user_id;

    conn.run(move |c| -> Result<_, Error> {

        let user: User = users::table.find(user_id.clone()).first(c)?;

        let role_ids: Vec<String> = UserRole::belonging_to(&user)
            .inner_join(roles::table.on(roles::role_id.eq(user_role::role_id)))
            .select(roles::role_id)
            .load(c)?;
        
        let user_has_access_to_subject: bool = diesel::select(exists(
            subjects::table
                .inner_join(subject_role::table.on(subject_role::subject_id.eq(subjects::subject_id)))
                .inner_join(assignments::table.on(assignments::subject_id.eq(subjects::subject_id)))
                .filter(subject_role::role_id.eq_any(&role_ids))
                .filter(assignments::assignment_id.eq(&assignment_id))
        )).get_result(c)?;

        if !user_has_access_to_subject {
            return Err(Error::Forbidden(()));
        }

        let solution_id = Uuid::new_v4().to_string();
        sln.solution_id = solution_id.clone();
        sln.submission_date = Utc::now().naive_utc();
        sln.assignment_id = assignment_id;

        let _result = diesel::insert_into(solutions::table)
            .values(sln)
            .execute(c)?;

        let user_solution = UserSolution { user_id, solution_id };
        let _result = diesel::insert_into(user_solution::table)
            .values(user_solution)
            .execute(c)?;

        Ok(())
    })
    .await?;

    Ok(Response::Ok(()))
}