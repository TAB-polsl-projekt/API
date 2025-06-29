use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::delete;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

use crate::admin_session::AdminSession;
use crate::schema::{subjects};
use crate::define_api_response;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: endpoint]
}

define_api_response!(pub enum Response {
    Ok => (200, "", (), ()),
});

define_api_response!(pub enum Error {
    InternalServerError => (500, "TEST", (), (diesel::result::Error)),
});

#[openapi(tag = "Subjects", operation_id = "deleteSubjects")]
#[delete("/subjects/<subject_id>")]
pub async fn endpoint(subject_id: String, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let _result = conn.run(move |c| {
        diesel::delete(subjects::table)
            .filter(subjects::subject_id.eq(subject_id))
            .execute(c)
    })
    .await?;

    Ok(Response::Ok(()))
}