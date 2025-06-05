use diesel::dsl::exists;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{put, http::CookieJar, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;
use diesel::prelude::*;

use crate::dbmodels::{Assignment, AssignmentUpdate, User};
use crate::dbschema::{assigments, session_refresh_keys, subjects, user_solution_assignments, user_subjects};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    Other(String)
    
}

#[openapi(tag = "Account")]
#[put("/assignments/<assignment_id>", data = "<assignment_update>")]
pub async fn endpoint(assignment_id: String, assignment_update: Json<AssignmentUpdate>, conn: crate::db::DbConn, jar: &CookieJar<'_>) -> Result<(), BadRequest<Json<Error>>> {
    let assignment_update = assignment_update.0;
    
    let session_id = jar.get("session_id").map(|cookie| cookie.value())
            .ok_or(Error::Other("Invalid session ID".to_string())).map_err(|e| BadRequest(Json(e)))?.to_string();

    conn.run(move |c| -> Result<_, Error> {

        let user_id: String = {
            session_refresh_keys::table
                .filter(session_refresh_keys::refresh_key_id.eq(session_id))
                .select(session_refresh_keys::user_id)
                .first(c)
                .map_err(|_e| Error::Other("Invalid session ID".to_string()))?
        };

        let is_user_editor_query = assigments::table
            .inner_join(subjects::table.on(subjects::subject_id.eq(assigments::subject_id.nullable())))
            .inner_join(user_subjects::table.on(user_subjects::role_id.eq(subjects::editor_role_id.nullable())))
            .filter(user_subjects::user_id.eq(user_id))
            .filter(assigments::assigment_id.eq(&assignment_id));

        let is_user_editor: bool = diesel::select(exists(is_user_editor_query))
            .get_result(c)
            .map_err(|_err| Error::Other("".to_string()))?;

        if !is_user_editor {
            return Err(Error::Other("User is not editor".to_string()));
        }

        let _rows_affected = diesel::update(
            assigments::table.filter(assigments::assigment_id.eq(&assignment_id))
        ).set(&assignment_update)
        .execute(c)
        .map_err(|_err| Error::Other("".to_string()))?;

        Ok(())
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}