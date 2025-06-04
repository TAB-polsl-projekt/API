use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, http::CookieJar, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;
use diesel::prelude::*;

use crate::dbmodels::{Assignment, User};
use crate::dbschema::{assigments, subjects, user_subjects};

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
#[get("/subjects/<subject_id>")]
pub async fn endpoint(subject_id: String, conn: crate::db::DbConn, jar: &CookieJar<'_>) -> Result<Json<Response>, BadRequest<Json<Error>>> {
    let session_id = jar.get("session_id").map(|cookie| cookie.value())
            .ok_or(Error::Other("Invalid session ID".to_string())).map_err(|e| BadRequest(Json(e)))?.to_string();

    conn.run(move |c| -> Result<_, Error> {

        let user_id: String = {
            use crate::dbschema::session_refresh_keys::dsl::*;
            
            session_refresh_keys
                .filter(refresh_key_id.eq(session_id))
                .select(user_id)
                .first(c)
                .map_err(|_e| Error::Other("Invalid session ID".to_string()))?
        };

        let assignments: Vec<_> = {
            subjects::table
                .inner_join(user_subjects::table.on(user_subjects::subject_id.eq(subjects::subject_id)))
                .inner_join(assigments::table.on(subjects::subject_id.eq(assigments::subject_id.nullable())))
                .filter(user_subjects::user_id.eq(user_id))
                .filter(subjects::subject_id.eq(subject_id))
                .select(assigments::all_columns)
                .get_results(c)
                .map_err(|_e| Error::Other("".to_string()))?
        };

        Ok(Json(Response { subject_name: "".to_string(), assignments }))
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}