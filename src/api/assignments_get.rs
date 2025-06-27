use diesel::RunQueryDsl;
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::dbmodels::Assignment;
use crate::schema::{assignments};
use crate::define_api_response;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "", Vec<Assignment>, ()),
});

define_api_response!(pub enum Error {
    Unauthorized => (401, "User is not an admin", (), ()),
    InternalServerError => (500, "TEST", String, (diesel::result::Error)),
});

#[openapi(tag = "Assignments", operation_id = "getAssignments")]
#[get("/assignments")]
pub async fn endpoint(conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let result = conn.run(move |c| {
        assignments::table.load(c)
    }).await?;

    Ok(Response::Ok(result))
}