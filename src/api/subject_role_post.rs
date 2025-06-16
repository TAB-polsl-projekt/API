use diesel::result::DatabaseErrorKind;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::post;
use rocket::{put, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::dbmodels::{Role, UserSubjects};
use crate::dbschema::{roles, user_subjects};
use crate::define_api_response;
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RequestData {
    pub role_id: String,
    pub user_id: String,
    pub subject_id: String,
}

define_api_response!(pub enum Response {
    Ok => (200, "OK", (), ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "User is not admin", (), ()),
    Conflict => (409, "Record already exists", (), ()),
    InternalServerError => (500, "TEST", (), (diesel::result::Error)),
});

#[openapi(tag = "Subjects", operation_id = "postRole")]
#[post("/subjects/add-role", data = "<data>")]
pub async fn endpoint(data: Json<RequestData>, conn: crate::db::DbConn, session: Session) -> Result<Response, Error> {
    let data = data.0;
    
    if !session.is_admin {
        return Err(Error::Unauthorized(()));
    }

    let user_id = data.user_id;
    let role_id = data.role_id;
    let subject_id = data.subject_id;
    let us = UserSubjects {
        user_id,
        subject_id,
        role_id,
        grade: None
    };
    
    let result = conn.run(move |c|{
        diesel::insert_into(user_subjects::table)
            .values(us)
            .execute(c)
    }).await;

    println!("{:?}", result);

    let _ = match result {
        Ok(val) => val,
        Err(err) => match err {
            diesel::result::Error::DatabaseError(kind, _) => match kind {
                DatabaseErrorKind::UniqueViolation => return Err(Error::Conflict(())),
                _ => return Err(Error::InternalServerError(()))
            }
            _ => return Err(Error::InternalServerError(())),
        }
    };

    Ok(Response::Ok(()))
}