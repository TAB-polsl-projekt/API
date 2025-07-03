use rocket::{get, FromForm};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::{define_api_response, define_response_data};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(enum Response {
    Ok => (200, "Ok", bool, ()),
});

define_api_response!(enum Error {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

define_response_data!(

#[derive(FromForm)]
    struct Query {
    user_id: String,
}

);

#[openapi(tag = "Assignments", operation_id = "getAssignmentAttendance")]
#[get("/assignments/<_assignment_id>/attendance?<_query..>")]
async fn endpoint(_query: Query, _assignment_id: String, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    todo!()
}
