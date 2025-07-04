use rocket::{delete, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{admin_session::AdminSession, define_api_response, schema::subject_role};
use diesel::{ExpressionMethods, RunQueryDsl};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RequestData {
    pub role_id: String,
    pub subject_id: String,
}

define_api_response!(pub enum Response {
    Ok => (200, "OK", (), ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

#[openapi(tag = "Subjects", operation_id = "deleteSubjectRole")]
#[delete("/subjects/add-role", data = "<data>")]
pub async fn endpoint(data: Json<RequestData>, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let data = data.0;

    let role_id = data.role_id;
    let subject_id = data.subject_id;
    
    let _ = conn.run(move |c|{
        diesel::delete(subject_role::table)
            .filter(subject_role::role_id.eq(role_id))
            .filter(subject_role::subject_id.eq(subject_id))
            .execute(c)
    }).await?;

    Ok(Response::Ok(()))
}