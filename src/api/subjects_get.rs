use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, http::CookieJar, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;
use diesel::prelude::*;

use crate::dbmodels::{Assignment, Subject, User};
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
    subject_id: String,
    subject_name: String,
}

#[openapi(tag = "Account")]
#[get("/subjects")]
pub async fn endpoint(conn: crate::db::DbConn, jar: &CookieJar<'_>) -> Result<Json<Vec<Response>>, BadRequest<Json<Error>>> {
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

        let subjects: Vec<Subject> = {
            subjects::table
                .inner_join(user_subjects::table.on(user_subjects::subject_id.eq(subjects::subject_id)))
                .filter(user_subjects::user_id.eq(user_id))
                .select(subjects::all_columns)
                .get_results(c)
                .map_err(|_e| Error::Other("".to_string()))?
        };

        let responses = subjects
            .iter()
            .map(|x| {
                    let subject_id = x.subject_id.clone().unwrap_or("".to_string());
                    let subject_name = x.subject_name.clone().unwrap_or("".to_string());

                    Response {
                        subject_id,
                        subject_name
                    }
                }   
            )
            .collect();

        Ok(Json(responses))
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}