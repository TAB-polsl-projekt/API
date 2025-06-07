use diesel::dsl::exists;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{post, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;
use diesel::prelude::*;

use crate::dbmodels::Assignment;
use crate::dbschema::{assigments, roles, user_subjects};
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    Other(String)
}

#[openapi(tag = "Account")]
#[post("/assignments", data = "<assignment>")]
pub async fn endpoint(assignment: Json<Assignment>, conn: crate::db::DbConn, session: Session) -> Result<(), BadRequest<Json<Error>>> {
    let user_id = session.user_id;

    let assignment = assignment.0;
    conn.run(move |c| -> Result<_, Error> {

        let is_user_admin_query = roles::table
            .inner_join(user_subjects::table.on(user_subjects::role_id.eq(roles::role_id)))
            .filter(roles::role_id.eq("0"))
            .filter(user_subjects::user_id.eq(user_id));

        let is_user_admin: bool = diesel::select(exists(is_user_admin_query))
            .get_result(c)
            .map_err(|_err| Error::Other("".to_string()))?;

        if !is_user_admin {
            return Err(Error::Other("User is not admin".to_string()));
        }
            
        let _result = diesel::insert_into(assigments::table)
            .values(assignment)
            .execute(c)
            .map_err(|_err| Error::Other("".to_string()))?;

        Ok(())
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}