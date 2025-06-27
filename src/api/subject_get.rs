use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};

use crate::dbmodels::{Assignment, Role, Subject, SubjectRole, User, UserRole};
use crate::schema::{subject_role, user_role};
use crate::schema::{roles, subjects, users};
use crate::define_api_response;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Debug, schemars::JsonSchema, Serialize, Deserialize)]
pub struct ResponseData {
    subject_name: String,
    assignments: Vec<Assignment>,
}

define_api_response!(pub enum Response {
    Ok => (200, "", ResponseData, ())
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "", (), (diesel::result::Error))
});

#[openapi(tag = "Subjects", operation_id = "getSubject")]
#[get("/subjects/<subject_id>")]
pub async fn endpoint(subject_id: String, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let user_id = session.user_id;

    conn.run(move |c| {

        let result = {
            let user: User = users::table.find(user_id).first(c)?;

            let roles: Vec<Role> = UserRole::belonging_to(&user)
                .inner_join(roles::table.on(roles::role_id.eq(user_role::role_id)))
                .select(roles::all_columns)
                .load(c)?;

            let subject: Subject = SubjectRole::belonging_to(&roles)
                .inner_join(subjects::table.on(subjects::subject_id.eq(subject_role::subject_id)))
                .filter(subjects::subject_id.eq(subject_id))
                .select(subjects::all_columns)
                .first(c)?;

            let assignments = Assignment::belonging_to(&subject).load(c)?;
            let subject_name = subject.subject_name;

            ResponseData {
                subject_name,
                assignments
            }
        };

        Ok(Response::Ok(result))
    }).await
}