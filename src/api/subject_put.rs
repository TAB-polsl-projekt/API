use diesel::{AsChangeset, ExpressionMethods, RunQueryDsl};
use rocket::{put, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::admin_session::AdminSession;
use crate::define_api_response;
use crate::schema::subjects;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: put_subject]
}

define_api_response!(enum PutSubjectResponse {
    Ok => (200, "Subject has been updated", (), ())
});

define_api_response!(enum PutSubjectError {
    InternalServerError => (500, "Unexpected server error", (), (diesel::result::Error))
});

#[derive(Deserialize, JsonSchema, AsChangeset)]
#[diesel(table_name = subjects)]
struct SubjectUpdate {
    pub subject_name: Option<String>,
    pub editor_role_id: Option<String>
}

#[openapi(tag = "Subjects", operation_id = "putSubject")]
#[put("/subjects/<subject_id>", data = "<subject_update>")]
async fn put_subject(subject_id: String, subject_update: Json<SubjectUpdate>, conn: crate::db::DbConn, _session: AdminSession) -> Result<PutSubjectResponse, PutSubjectError> {
    let subject_update = subject_update.0;

    let _ = conn.run(move |c| {
        diesel::update(subjects::table)
            .filter(subjects::subject_id.eq(subject_id))
            .set(&subject_update)
            .execute(c)
    }).await?;

    Ok(PutSubjectResponse::Ok(()))
}
