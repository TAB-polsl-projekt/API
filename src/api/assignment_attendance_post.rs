use chrono::NaiveDateTime;
use rocket::post;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::{define_api_response, define_response_data};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: post_assignment_attendance]
}

define_api_response!(enum Response {
    //Ok => (200, "Attendance has been set", (), ()),
});

define_api_response!(enum Error {
    //InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error)),
});

define_response_data!(struct AttendanceData {
    date: NaiveDateTime,
    user_id: String
});

#[openapi(tag = "Assignments", operation_id = "postAssignmentAttendance")]
#[post("/assignments/{_assignment_id}/attendance", data = "<_attendance_data>")]
async fn post_assignment_attendance(_assignment_id: String, _attendance_data: AttendanceData, _conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    todo!();
}