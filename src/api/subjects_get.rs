use diesel::{BelongingToDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};

use crate::dbmodels::{Role, Subject, SubjectRole, User, UserRole};
use crate::schema::{roles, subject_role, subjects, user_role, users};
use crate::define_api_response;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Debug, schemars::JsonSchema, Serialize, Deserialize)]
pub struct ResponseData {
    subject_id: String,
    subject_name: String,
}

define_api_response!(pub enum Response {
    Ok => (200, "", Vec<ResponseData>, ())
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "", (), (diesel::result::Error))
});

#[openapi(tag = "Subjects", operation_id = "getSubjects")]
#[get("/subjects")]
pub async fn endpoint(conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let user_id = session.user_id;
    let is_admin = session.is_admin;

    conn.run(move |c| -> Result<_, Error> {

        let subjects: Vec<Subject> = {
            if !is_admin {
                let user: User = users::table.find(user_id).first(c)?;

                let roles: Vec<Role> = UserRole::belonging_to(&user)
                    .inner_join(roles::table.on(roles::role_id.eq(user_role::role_id)))
                    .select(roles::all_columns)
                    .load(c)?;

                let subjects = SubjectRole::belonging_to(&roles)
                    .inner_join(subjects::table.on(subjects::subject_id.eq(subject_role::subject_id)))
                    .select((subjects::subject_id, subjects::subject_name))
                    .load(c)?;

                subjects
            } else {
                subjects::table.load(c)?
            }
        };

        let responses = subjects
            .iter()
            .map(|x| {
                    let subject_id = x.subject_id.clone();
                    let subject_name = x.subject_name.clone();

                    ResponseData {
                        subject_id,
                        subject_name
                    }
                }   
            )
            .collect();

        Ok(
            Response::Ok(
                responses
            )
        )
    }).await
}