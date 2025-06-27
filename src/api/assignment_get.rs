use diesel::dsl::exists;
use diesel::{select, BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::dbmodels::{Assignment, Subject, User, UserRole};
use crate::schema::{assignments, roles, subject_role, subjects, user_role, users};
use crate::define_api_response;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "TEST", Assignment, ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "User does not have access to the asignment", (), ()),
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

#[openapi(tag = "Account")]
#[get("/assignments/<assignment_id>")]
pub async fn endpoint(assignment_id: String, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let user_id = session.user_id;
    
    let result = conn.run(move |c| {
        let user: User = users::table.find(user_id).first(c)?;

        let assignment: Assignment = assignments::table.find(assignment_id).first(c)?;

        let subject: Subject = subjects::table.find(&assignment.subject_id).first(c)?;

        let role_ids: Vec<String> = UserRole::belonging_to(&user)
            .inner_join(roles::table.on(roles::role_id.eq(user_role::role_id)))
            .select(roles::role_id)
            .load(c)?;

        let does_user_have_access: bool = select(exists(
            subject_role::table
                .filter(subject_role::subject_id.eq(subject.subject_id))
                .filter(subject_role::role_id.eq_any(role_ids))
        )).get_result(c)?;

        if !does_user_have_access {
            return Err(Error::Unauthorized(()));
        }

        Ok(assignment)
    }).await?;

    Ok(Response::Ok(result))
}