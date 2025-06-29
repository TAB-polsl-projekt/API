use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::delete;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::define_api_response;
use crate::schema::assignments;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "", (), ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "TEST", (), (diesel::result::Error)),
});

#[openapi(tag = "Assignments", operation_id = "deleteAssignment")]
#[delete("/assignments/<assignment_id>")]
pub async fn endpoint(assignment_id: String, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let _result = conn.run(move |c| {
        diesel::delete(assignments::table)
            .filter(assignments::assignment_id.eq(assignment_id))
            .execute(c)
    })
    .await?;

    Ok(Response::Ok(()))
}