use diesel::RunQueryDsl;
use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use uuid::Uuid;

use crate::admin_session::AdminSession;
use crate::dbmodels::Subject;
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

#[openapi(tag = "Subjects", operation_id = "postSubjects")]
#[post("/subjects", data = "<subject>")]
pub async fn endpoint(subject: Json<Subject>, conn: crate::db::DbConn, _session: AdminSession) -> Result<Response, Error> {
    let mut subject = subject.0;

    subject.subject_id = Uuid::new_v4().to_string();

    let _result = conn.run(move |c| {
        diesel::insert_into(subjects::table)
            .values(subject)
            .execute(c)
    })
    .await?;

    Ok(Response::Ok(()))
}