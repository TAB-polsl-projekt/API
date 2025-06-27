use diesel::result::DatabaseErrorKind;
use diesel::RunQueryDsl;
use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::SubjectRole;
use crate::define_api_response;
use crate::schema::subject_role;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "OK", (), ()),
});

define_api_response!(pub enum Error {
    Conflict => (409, "Record already exists", (), ()),
    InternalServerError => (500, "TEST", (), (diesel::result::Error)),
});

#[openapi(tag = "Subjects", operation_id = "postRole")]
#[post("/subjects/add-role", data = "<data>")]
pub async fn endpoint(data: Json<SubjectRole>, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let data = data.0;
    
    let result = conn.run(move |c|{
        diesel::insert_into(subject_role::table)
            .values(data)
            .execute(c)
    }).await;

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