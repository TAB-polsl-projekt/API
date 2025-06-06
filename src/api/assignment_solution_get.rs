use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, http::CookieJar, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;
use diesel::prelude::*;
use crate::dbschema::assigments::dsl::assigments;

use crate::dbmodels::Assignment;
use crate::dbschema::{solution, user_subjects};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    Other(String)
    
}

pub type Result = Assignment;

#[openapi(tag = "Assignments")]
#[get("/assignments/solution")]
pub async fn endpoint(assignment_id: String, conn: crate::db::DbConn, jar: &CookieJar<'_>) -> Result<Json<Response>, BadRequest<Json<Error>>> {
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

        let result: Vec<Assignment> = {
            let aid = assignment_id;


            assigments::table
                .inner_join(user_subjects.on(user_subjects::user_id.eq(user_id)))
                .filter(user_id.eq(uid))
                .filter(assigment_id.eq(aid))
                .order(solution::submission_date.desc())
                .select(assigments::all_columns)
                .get_results(c)
                .map_err(|_e| Error::Other("Failed to execute a query".to_string()))?
        };

        let grade = grade.unwrap_or(0.0);

        Ok(Json(Response { grade }))
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}