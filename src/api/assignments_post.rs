use diesel::RunQueryDsl;
use rocket::{post, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::Assignment;
use crate::define_api_response;
use crate::schema::assignments;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "", String, ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "User is not an admin", String, ()),
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

#[openapi(tag = "Assignments", operation_id = "postAssignment")]
#[post("/assignments", data = "<assignment>")]
pub async fn endpoint(assignment: Json<Assignment>, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let assignment = assignment.0;

    let _result = conn.run(move |c| {
        diesel::insert_into(assignments::table)
            .values(assignment)
            .execute(c)
    }).await?;

    Ok(Response::Ok("".to_owned()))
}