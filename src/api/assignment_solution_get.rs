use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{get, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use serde::{Deserialize, Serialize};
use rocket::response::status::BadRequest;
use diesel::prelude::*;

use crate::dbmodels::Solution;
use crate::dbschema::{assigments, solution, user_solution_assignments};
use crate::session::Session;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Error {
    Other(String)
    
}

pub type Response = Solution;

#[openapi(tag = "Assignments")]
#[get("/assignments/<assignment_id>/solution")]
pub async fn endpoint(assignment_id: String, conn: crate::db::DbConn, session: Session) -> Result<Json<Response>, BadRequest<Json<Error>>> {
    let user_id = session.user_id;

    conn.run(move |c| -> Result<_, Error> {

        let solution: Solution = {

            assigments::table
                .inner_join(user_solution_assignments::table.on(user_solution_assignments::assigment_id.eq(assigments::assigment_id)))
                .inner_join(solution::table.on(solution::solution_id.eq(user_solution_assignments::solution_id)))
                .filter(user_solution_assignments::user_id.eq(user_id))
                .filter(assigments::assigment_id.eq(assignment_id))
                .order(solution::submission_date.desc())
                .select(solution::all_columns)
                .first(c)
                .map_err(|_e| Error::Other("Failed to execute a query".to_string()))?
        };

        Ok(Json(solution))
    })
    .await
    .map_err(|e| BadRequest(Json(e)))
}