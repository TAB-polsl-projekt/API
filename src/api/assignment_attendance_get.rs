use rand::Rng;
use rocket::get;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::define_api_response;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "Ok", bool, ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

#[openapi(tag = "Assignments", operation_id = "getAssignmentAttendance")]
#[get("/assignments/{assignment_id}/attendance")]
pub async fn endpoint(conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let attendance = conn.run(move |c| {
        let mut rng = rand::rng();

        rng.random()
    }).await;

    Ok(Response::Ok(attendance))
}