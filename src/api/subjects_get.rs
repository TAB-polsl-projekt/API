use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::{get, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};

use crate::dbmodels::Subject;
use crate::dbschema::{subjects, user_subjects};
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
                subjects::table
                .inner_join(user_subjects::table.on(user_subjects::subject_id.eq(subjects::subject_id)))
                .filter(user_subjects::user_id.eq(user_id))
                .select(subjects::all_columns)
                .get_results(c)?
            } else {
                subjects::table
                .inner_join(user_subjects::table.on(user_subjects::subject_id.eq(subjects::subject_id)))
                .select(subjects::all_columns)
                .get_results(c)?
            }
        };

        let responses = subjects
            .iter()
            .map(|x| {
                    let subject_id = x.subject_id.clone();
                    let subject_name = x.subject_name.clone().unwrap_or("".to_string());

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