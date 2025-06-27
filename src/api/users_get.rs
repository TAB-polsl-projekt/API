use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::{get, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::{Role, Subject, SubjectRole, User, UserRole};
use crate::schema::{roles, subject_role, subjects, user_role, users};
use crate::define_api_response;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "TEST", Vec<User>, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "Internal server error", String, (diesel::result::Error)),
});

/// Gets all users enrolled in a subject
#[openapi(tag = "Account", operation_id = "getUsersEnrolledInSubject")]
#[get("/subjects/<subject_id>/users/enrolled")]
pub async fn endpoint(subject_id: String, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let result = conn.run(move |c| {
        let subject: Subject = subjects::table.find(subject_id).first(c)?;

        let roles: Vec<Role> = SubjectRole::belonging_to(&subject)
            .inner_join(roles::table.on(roles::role_id.eq(subject_role::role_id)))
            .select(roles::all_columns)
            .load(c)?;

        let users = UserRole::belonging_to(&roles)
            .inner_join(users::table.on(users::user_id.eq(user_role::user_id)))
            .select(users::all_columns)
            .load(c);

        users
    }).await?;

    Ok(Response::Ok(result))
}