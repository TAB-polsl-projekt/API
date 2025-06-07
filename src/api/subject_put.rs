use rocket::{put, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;

use crate::dbmodels::{Assignment, SubjectUpdate};
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    Other(String)
    
}

#[derive(Debug, schemars::JsonSchema, Serialize, Deserialize)]
pub struct Response {
    subject_name: String,
    assignments: Vec<Assignment>,
}

#[openapi(tag = "Account")]
#[put("/subjects/<subject_id>", data = "<update>")]
pub async fn endpoint(subject_id: String, update: Json<SubjectUpdate>, conn: crate::db::DbConn, session: Session) -> Result<(), BadRequest<Json<Error>>> {
    let update = update.0;
    let user_id = session.user_id;

    conn.run(move |c| -> Result<_, Error> {
        todo!()
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}