use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use rocket::{get, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;

use crate::dbmodels::Assignment;
use crate::dbschema::{assigments, user_solution_assignments};
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    Other(String)
    
}

pub type Response = Assignment;

#[openapi(tag = "Account")]
#[get("/assignments/<assignment_id>")]
pub async fn endpoint(assignment_id: String, conn: crate::db::DbConn, session: Session) -> Result<Json<Response>, BadRequest<Json<Error>>> {
    let user_id = session.user_id;
    
    conn.run(move |c| -> Result<_, Error> {

        let assignment: Assignment = {
            assigments::table
                .inner_join(user_solution_assignments::table.on(user_solution_assignments::assigment_id.eq(assigments::assigment_id)))
                .filter(user_solution_assignments::user_id.eq(user_id))
                .select(assigments::all_columns)
                .get_result(c)
                .map_err(|_e| Error::Other("".to_string()))?
        };

        Ok(Json(assignment))
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}